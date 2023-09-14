use ethers::signers::LocalWallet;
use hyperliquid::{Chain, Exchange, Hyperliquid};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: LocalWallet = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
        .parse()
        .unwrap();

    let exchange: Exchange = Hyperliquid::new(wallet, Chain::Dev);

    let destination = "0x0D1d9635D0640821d15e323ac8AdADfA9c111414"
        .parse()
        .expect("Invalid address");

    let amount = "1".to_string(); // USD

    println!("Transferring 1 USDC to {destination} ...");

    let res = exchange.usdc_transfer(destination, amount).await.unwrap();

    println!("Response: {:?}", res);
}
