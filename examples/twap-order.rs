use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{
    types::{exchange::request::TwapRequest, Chain},
    utils::parse_size,
    Exchange, Hyperliquid,
};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::Dev);

    let asset = 0;
    let sz_decimals = 2;

    let twap = TwapRequest {
        asset,
        is_buy: true,
        reduce_only: false,
        duration: 10,
        sz: parse_size(13.85, sz_decimals),
        randomize: true,
    };

    let vault_address = None;

    println!("Placing a TWAP order...");
    let response = exchange
        .twap_order(wallet.clone(), twap, vault_address)
        .await
        .expect("Failed to place twap order");

    println!("Response: {:?}", response);
}
