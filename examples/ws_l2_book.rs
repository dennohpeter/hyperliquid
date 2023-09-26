use hyperliquid::{
    types::{
        exchange::request::Chain,
        websocket::{
            request::{Channel, Subscription},
            response::Response,
        },
    },
    Hyperliquid, Result, Websocket,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut ws: Websocket = Hyperliquid::new(Chain::Dev);

    ws.connect().await?;

    let books = Channel {
        id: 3,
        sub: Subscription::L2Book { coin: "BTC".into() },
    };

    ws.subscribe(&[books]).await?;

    let handler = |event: Response| {
        println!("Received L2 Books: \n--\n{:?}", event);

        Ok(())
    };

    ws.next(handler).await?;

    ws.disconnect().await?;

    Ok(())
}
