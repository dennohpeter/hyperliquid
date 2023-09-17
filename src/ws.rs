use std::{collections::HashMap, sync::Arc};

use ethers::signers::LocalWallet;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    error::{Error, Result},
    types::ws::{Event, Subscription},
    Method, Request,
};

pub struct Websocket {
    pub stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub channels: HashMap<u64, Subscription>,
    pub wallet: Arc<LocalWallet>,
    pub url: String,
}

impl Websocket {
    /// Returns `true` if the websocket is connected
    pub async fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    /// Connect to the websocket
    pub async fn connect(&mut self) -> Result<()> {
        let (stream, _) = connect_async(&self.url).await?;
        self.stream = Some(stream);

        Ok(())
    }

    /// Disconnect from the websocket
    pub async fn disconnect(&mut self) -> Result<()> {
        // let _res = self.send(&[()], false).await;

        self.stream = None;

        Ok(())
    }

    /// Subscribe to the given channels
    /// - `channels` - The channels to subscribe to
    pub async fn subscribe(&mut self, channels: Vec<(Subscription, u64)>) -> Result<()> {
        self.send(&channels, true).await?;

        channels.into_iter().for_each(|(channel, id)| {
            self.channels.insert(id, channel);
        });

        Ok(())
    }

    /// Unsubscribe from the given channels
    /// - `channels` - The channels to unsubscribe from
    pub async fn unsubscribe(&mut self, channels: &Vec<(Subscription, u64)>) -> Result<()> {
        for (_, id) in channels {
            if !self.channels.contains_key(id) {
                return Err(Error::NotSubscribed(*id));
            }
        }

        self.send(channels, false).await?;

        channels.into_iter().for_each(|(_, id)| {
            self.channels.remove(id);
        });

        Ok(())
    }

    /// Unsubscribe from all channels
    pub async fn unsubscribe_all(&mut self) -> Result<()> {
        let channels: Vec<(Subscription, u64)> = self
            .channels
            .clone()
            .into_iter()
            .map(|(id, channel)| (channel, id))
            .collect();

        self.send(&channels, false).await
    }

    /// Send a message request
    /// - `subscriptions` is a list of subscriptions to send
    pub async fn send(
        &mut self,
        subscriptions: &Vec<(Subscription, u64)>,
        subscribe: bool,
    ) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            for (subscription, _) in subscriptions {
                let method = if subscribe {
                    Method::Subscribe
                } else {
                    Method::Unsubscribe
                };

                let request = Request {
                    method,
                    subscription: subscription.clone(),
                };

                let message = Message::Text(serde_json::to_string(&request)?);

                stream.send(message).await?;
            }

            return Ok(());
        }

        Err(Error::NotConnected)
    }

    pub async fn next<Callback>(&mut self, handler: Callback) -> Result<Option<bool>>
    where
        Callback: Fn(Event) -> Result<()>,
    {
        if let Some(stream) = &mut self.stream {
            while let Some(message) = stream.next().await {
                let message = message?;

                if let Message::Text(text) = message {
                    if !text.starts_with('{') {
                        continue;
                    }
                    let event = serde_json::from_str(&text)?;

                    (handler)(event)?;
                }
            }
        }

        Ok(None)
    }
}
