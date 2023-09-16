use std::sync::Arc;

use ethers::{signers::LocalWallet, types::Address};
use hyperliquid::{Chain, Hyperliquid, Info};

const SEP: &str = "\n---";

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: LocalWallet = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
        .parse()
        .unwrap();

    let user: Address = "0xc64cc00b46101bd40aa1c3121195e85c0b0918d8"
        .parse()
        .expect("Invalid address");

    let info = Hyperliquid::new(Arc::new(wallet), Chain::Dev);

    println!("Info API Examples");

    metadata(&info).await;
    mids(&info).await;
    contexts(&info).await;
    user_state(&info, user).await;
    open_orders(&info, user).await;
    user_fills(&info, user).await;
    user_funding(&info, user).await;
    funding_history(&info).await;
    l2_book(&info).await;
    candle_snapshot(&info).await;
}

async fn metadata(info: &Info) {
    let metadata = info.metadata().await.unwrap();
    println!("{SEP}\nMetadata \n{:?}{SEP}", metadata.universe);
}

async fn mids(info: &Info) {
    let mids = info.mids().await.unwrap();
    println!("Mids \n{:?}{SEP}", mids);
}

async fn contexts(info: &Info) {
    let contexts = info.contexts().await.unwrap();
    println!("Asset Contexts \n{:?}{SEP}", contexts);
}

async fn user_state(info: &Info, user: Address) {
    let user_state = info.user_state(user).await.unwrap();
    println!("User state for {user} \n{:?}{SEP}", user_state);
}

async fn open_orders(info: &Info, user: Address) {
    let open_orders = info.open_orders(user).await.unwrap();
    println!("Open orders for {user} \n{:?}{SEP}", open_orders);
}

async fn user_fills(info: &Info, user: Address) {
    let user_fills = info.user_fills(user).await.unwrap();
    println!("User fills for {user} \n{:?}{SEP}", user_fills);
}

async fn user_funding(info: &Info, user: Address) {
    let start_timestamp = 1690540602225;
    let end_timestamp = 1690569402225;

    let user_funding = info
        .user_funding(user, start_timestamp, Some(end_timestamp))
        .await
        .unwrap();
    println!(
        "User funding for {user} between {start_timestamp} and {end_timestamp} \n{:?}{SEP}",
        user_funding
    );
}

async fn funding_history(info: &Info) {
    let coin = "ETH";

    let start_timestamp = 1690540602225;
    let end_timestamp = 1690569402225;

    let funding_history = info
        .funding_history(coin.to_string(), start_timestamp, Some(end_timestamp))
        .await
        .unwrap();
    println!(
        "Funding history for {coin} between {start_timestamp} and {end_timestamp} \n{:?}{SEP}",
        funding_history
    );
}

async fn l2_book(info: &Info) {
    let coin = "ETH";

    let l2_book = info.l2_book(coin.to_string()).await.unwrap();
    println!("L2 book for {coin} \n{:?}{SEP}", l2_book);
}

async fn candle_snapshot(info: &Info) {
    let coin = "ETH";
    let interval = "15m";
    let start_timestamp = 1690540602225;
    let end_timestamp = 1690569402225;

    let snapshot = info
        .candle_snapshot(
            coin.to_string(),
            interval.to_string(),
            start_timestamp,
            end_timestamp,
        )
        .await
        .unwrap();
    println!("Candle snapshot for {coin} between {start_timestamp} and {end_timestamp} with interval {interval} \n{:?}{SEP}",snapshot);
}
