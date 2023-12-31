use std::sync::Arc;

use ethers::signers::{LocalWallet, Signer};
use hyperliquid::{types::exchange::request::Chain, Exchange, Hyperliquid, Info};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::Dev);

    let leverage = 5;
    let asset = 4;
    let is_cross = false;

    println!("Updating leverage to {} ...", leverage);

    let res = exchange
        .update_leverage(wallet.clone(), leverage, asset, is_cross)
        .await
        .unwrap();

    println!("Response: {:?}", res);

    println!("--\nUpdating isolated margin to for ETH to 1% ...");

    let margin = 1;
    let res = exchange
        .update_isolated_margin(wallet.clone(), margin, asset)
        .await
        .unwrap();

    println!("Response: {:?}", res);

    let info: Info = Hyperliquid::new(Chain::Dev);

    // user state
    let res = info.user_state(wallet.address()).await.unwrap();

    println!("--\nUser state: {:?}", res);
}
