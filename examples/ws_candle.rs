use hyperliquid::{
    types::{
        websocket::{
            request::{Channel, Subscription},
            response::Response,
        },
        Chain,
    },
    Hyperliquid, Result, Websocket,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut ws: Websocket = Hyperliquid::new(Chain::Dev);

    ws.connect().await?;

    let candle = Channel {
        id: 2,
        sub: Subscription::Candle {
            coin: "BTC".into(),
            interval: "5m".into(),
        },
    };

    ws.subscribe(&[candle]).await?;

    let handler = |event: Response| async move {
        println!("Received Candle: \n--\n{:?}", event);

        Ok(())
    };

    ws.next(handler).await?;

    ws.disconnect().await?;

    Ok(())
}
