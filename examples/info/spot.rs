use std::sync::Arc;

use ethers::{
    signers::{LocalWallet, Signer},
    types::Address,
};
use hyperliquid::{types::Chain, Hyperliquid, Info};

const SEP: &str = "\n---";

#[tokio::main]
pub async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let user = wallet.address();

    let info = Hyperliquid::new(Chain::Dev);

    println!("Info Spot API Examples");

    spot_meta(&info).await;
    spot_meta_and_asset_ctxs(&info).await;
    spot_clearinghouse_state(&info, user).await;
}

async fn spot_meta(info: &Info) {
    let spot_meta = info.spot_meta().await.unwrap();
    println!("{SEP}\nSpot Metadata \n{:?}{SEP}", spot_meta);
}

async fn spot_meta_and_asset_ctxs(info: &Info) {
    let spot_asset_ctxs = info.spot_meta_and_asset_ctxs().await.unwrap();
    println!("Spot Asset Contexts \n{:?}{SEP}", spot_asset_ctxs);
}

async fn spot_clearinghouse_state(info: &Info, user: Address) {
    let states = info.spot_clearinghouse_state(user).await.unwrap();
    println!("User spot state for {user} \n{:?}{SEP}", states);
}
