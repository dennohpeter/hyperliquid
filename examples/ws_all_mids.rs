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

    let subscription = Channel {
        id: 1,
        sub: Subscription::AllMids,
    };

    ws.subscribe(&[subscription]).await?;

    let handler = |event: Response| async move {
        println!("Received All Mids: \n--\n{:?}", event);

        Ok(())
    };

    ws.next(handler).await?;

    ws.disconnect().await?;

    Ok(())
}
