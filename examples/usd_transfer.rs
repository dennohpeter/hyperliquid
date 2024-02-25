use std::sync::Arc;

use ethers::signers::{LocalWallet, Signer};
use hyperliquid::{types::Chain, Exchange, Hyperliquid};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::ArbitrumTestnet);

    let destination = "0x0D1d9635D0640821d15e323ac8AdADfA9c111414"
        .parse()
        .expect("Invalid address");

    let amount = "1".to_string(); // USD

    println!(
        "Transferring ${} from {:?} to {:?}",
        amount,
        wallet.address(),
        destination
    );

    let res = exchange
        .usdc_transfer(wallet.clone(), destination, amount)
        .await
        .unwrap();

    println!("Response: {:?}", res);
}
