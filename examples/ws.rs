use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{Chain, Event, Hyperliquid, Result, Subscription, Websocket};

#[tokio::main]
async fn main() -> Result<()> {
    trades().await?;

    Ok(())
}

async fn trades() -> Result<()> {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let mut ws: Websocket = Hyperliquid::new(wallet.clone(), Chain::Dev);

    ws.connect().await?;

    let all_mids = (Subscription::AllMids, 1);
    let trades = (Subscription::Trades { coin: "BTC".into() }, 2);

    ws.subscribe(vec![all_mids, trades]).await?;

    ws.unsubscribe(&vec![(Subscription::AllMids, 2)]).await?;

    let handler = |event: Event| {
        println!("{:?}", event);

        Ok(())
    };

    ws.next(handler).await?;

    ws.disconnect().await?;
    Ok(())
}
