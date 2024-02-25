use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{types::Chain, Exchange, Hyperliquid};

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

    let usd = "10".to_string(); // USD

    println!("Withdrawing ${} from bridge to {:?}", usd, destination);

    let res = exchange
        .withdraw_from_bridge(wallet.clone(), destination, usd)
        .await
        .unwrap();

    println!("Response: {:?}", res);
}
