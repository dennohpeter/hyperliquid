use std::sync::Arc;

use ethers::signers::{LocalWallet, Signer};
use hyperliquid::{types::Chain, Exchange, Hyperliquid, Info};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::Dev);

    let leverage = 2;
    let asset = 4;
    let is_cross = false;

    println!("Updating leverage to {}x ...", leverage);

    let res = exchange
        .update_leverage(wallet.clone(), leverage, asset, is_cross)
        .await
        .unwrap();

    println!("Response: {:?}", res);

    let margin = 1;

    println!("--\nUpdating isolated margin for ETH to {margin}% ...");

    let res = exchange
        .update_isolated_margin(wallet.clone(), asset, true, margin)
        .await
        .unwrap();

    println!("Response: {:?}", res);

    let info: Info = Hyperliquid::new(Chain::Dev);

    // user state
    let res = info.user_state(wallet.address()).await.unwrap();

    println!("--\nUser state: {:?}", res);
}
