use std::sync::Arc;

use ethers::signers::{LocalWallet, Signer};
use hyperliquid::{types::exchange::request::Chain, Exchange, Hyperliquid};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "8547bf37e4ac35e85d1e8afc2a2ba5c7f352b8a11ae916e9f14737737e8e0e47"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::ArbitrumGoerli);

    let destination = "0x0D1d9635D0640821d15e323ac8AdADfA9c111414"
        .parse()
        .expect("Invalid address");

    let amount = "1".to_string(); // USD

    println!(
        "Transferring from {} 1 USDC to {destination} ...",
        wallet.address()
    );

    let res = exchange
        .usdc_transfer(wallet.clone(), destination, amount)
        .await
        .unwrap();

    println!("Response: {:?}", res);
}
