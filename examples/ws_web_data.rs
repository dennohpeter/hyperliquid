use std::sync::Arc;

use ethers::signers::{LocalWallet, Signer};
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
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let mut ws: Websocket = Hyperliquid::new(Chain::Dev);

    ws.connect().await?;

    let web_data = Channel {
        id: 2,
        sub: Subscription::WebData {
            user: wallet.address(),
        },
    };

    ws.subscribe(&[web_data]).await?;

    let handler = |event: Response| async move {
        println!("Received Web Data: \n--\n{:?}", event);

        Ok(())
    };

    ws.next(handler).await?;

    ws.disconnect().await?;

    Ok(())
}
