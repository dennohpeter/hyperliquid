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

    let trades = Channel {
        id: 2,
        sub: Subscription::Trades { coin: "BTC".into() },
    };

    ws.subscribe(&[trades]).await?;

    let handler = |event: Response| async move {
        println!("Received Trades: \n--\n{:?}", event);

        Ok(())
    };

    ws.next(handler).await?;

    ws.disconnect().await?;

    Ok(())
}
