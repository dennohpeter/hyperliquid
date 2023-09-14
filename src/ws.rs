use std::sync::atomic::{AtomicBool, Ordering};

use error_chain::bail;
use ethers::providers::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::Response, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::{errors::Result, types::ws::Event};

pub struct Websocket {
    socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(Event) -> Result<()>>,
}

impl Websocket {
    pub fn new<C>(handler: C) -> Self
    where
        C: FnMut(Event) -> Result<()> + 'static,
    {
        Self {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub async fn connect(&mut self, url: &str) -> Result<()> {
        self.connect_wss(url).await
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some((socket, _)) = &mut self.socket {
            let _ = socket.close(None).await;

            return Ok(());
        }

        bail!("No socket to disconnect");
    }

    async fn connect_wss(&mut self, url: &str) -> Result<()> {
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);

                Ok(())
            }
            Err(e) => {
                bail!("Error during handshake: {}", e);
            }
        }
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        if let Some((socket, _)) = &mut self.socket {
            let (_, mut reader) = socket.split();

            while running.load(Ordering::Relaxed) {
                let msg = reader
                    .next()
                    .await
                    .ok_or("Reader data not found")?
                    .map_err(|e| format!("Failed to read websocket message: {}", e))?;

                match msg {
                    Message::Text(t) => {
                        let event: Event = serde_json::from_str(&t).map_err(|e| {
                            format!("Failed to deserialize websocket text: {t:?} ({e})")
                        })?;

                        (self.handler)(event)?;
                    }
                    Message::Ping(_) => {
                        // writer.write_message(Message::Pong(vec![]))?;
                        // writer.send(Message::Pong(vec![])).await?;
                        println!("Ping");
                    }
                    Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                    Message::Close(e) => bail!("Disconnected: {:?}", e),
                }
            }
        }

        Ok(())
    }
}
