use std::collections::HashMap;

use futures_util::{Future, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    error::{Error, Result},
    types::websocket::{
        request::{Channel, Method, Request},
        response::Response,
    },
};

pub struct Websocket {
    pub stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub channels: HashMap<u64, Channel>,
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
        self.unsubscribe_all().await?;

        self.stream = None;
        Ok(())
    }

    /// Subscribe to the given channels
    /// - `channels` - The channels to subscribe to
    pub async fn subscribe(&mut self, channels: &[Channel]) -> Result<()> {
        self.send(channels, true).await?;

        channels.iter().for_each(|channel| {
            self.channels.insert(channel.id, channel.clone());
        });

        Ok(())
    }

    /// Unsubscribe from the given channels
    /// - `channels` - The channels to unsubscribe from
    pub async fn unsubscribe(&mut self, ids: &[u64]) -> Result<()> {
        let channels = ids
            .iter()
            .map(|id| {
                self.channels
                    .get(id)
                    .ok_or_else(|| Error::NotSubscribed(*id))
                    .cloned()
            })
            .collect::<Result<Vec<Channel>>>()?;

        self.send(&channels, false).await?;

        channels.iter().for_each(|channel| {
            self.channels.remove(&channel.id);
        });

        Ok(())
    }

    /// Unsubscribe from all channels
    pub async fn unsubscribe_all(&mut self) -> Result<()> {
        let channels: Vec<Channel> = self.channels.values().cloned().collect();

        self.send(&channels, false).await
    }

    pub async fn next<F, Fut>(&mut self, handler: F) -> Result<Option<bool>>
    where
        F: Fn(Response) -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        if let Some(stream) = &mut self.stream {
            while let Some(message) = stream.next().await {
                if let Message::Text(text) = message? {
                    let response = serde_json::from_str(&text)?;

                    (handler)(response).await?;
                }
            }
        }

        Ok(None)
    }

    /// Send a message request
    /// - `channels` is a list of subscriptions to send
    /// - `subscribe` is a boolean indicating whether to subscribe or unsubscribe
    async fn send(&mut self, channels: &[Channel], subscribe: bool) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            for channel in channels {
                let method = if subscribe {
                    Method::Subscribe
                } else {
                    Method::Unsubscribe
                };

                let request = Request {
                    method,
                    subscription: channel.sub.clone(),
                };

                let message = Message::Text(serde_json::to_string(&request)?);

                stream.send(message).await?;
            }

            return Ok(());
        }

        Err(Error::NotConnected)
    }
}
