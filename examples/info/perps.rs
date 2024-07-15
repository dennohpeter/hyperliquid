use std::{sync::Arc, time::SystemTime};

use ethers::{
    signers::{LocalWallet, Signer},
    types::Address,
};
use hyperliquid::{
    types::{
        exchange::request::{Limit, OrderRequest, OrderType, Tif},
        Chain, Oid,
    },
    utils::{parse_price, parse_size},
    Exchange, Hyperliquid, Info,
};
use uuid::Uuid;

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

    let info = Hyperliquid::new(Chain::ArbitrumTestnet);

    let exchange = Hyperliquid::new(Chain::ArbitrumTestnet);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");

    let now = now.as_millis() as u64;

    println!("Info Perps API Examples");

    metadata(&info).await;
    mids(&info).await;
    contexts(&info).await;
    user_state(&info, user).await;
    batch_user_states(&info, vec![user]).await;
    open_orders(&info, user).await;
    frontend_open_orders(&info, user).await;
    user_fills(&info, user).await;
    user_fills_by_time(&info, user, now - 1000000, Some(now)).await;
    user_funding(&info, user).await;
    funding_history(&info).await;
    l2_book(&info).await;
    recent_trades(&info).await;
    candle_snapshot(&info).await;
    order_status(&info, &exchange, wallet).await;
    sub_accounts(&info, user).await;
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

async fn batch_user_states(info: &Info, users: Vec<Address>) {
    let user_states = info.user_states(users.clone()).await.unwrap();
    println!("User states for {:?} \n{:?}{SEP}", users, user_states);
}

async fn open_orders(info: &Info, user: Address) {
    let open_orders = info.open_orders(user).await.unwrap();
    println!("Open orders for {user} \n{:?}{SEP}", open_orders);
}

async fn frontend_open_orders(info: &Info, user: Address) {
    let open_orders = info.frontend_open_orders(user).await.unwrap();
    println!("Frontend Open orders for {user} \n{:?}{SEP}", open_orders);
}

async fn user_fills(info: &Info, user: Address) {
    let user_fills = info.user_fills(user).await.unwrap();
    println!("User fills for {user} \n{:?}{SEP}", user_fills);
}

async fn user_fills_by_time(info: &Info, user: Address, start_time: u64, end_time: Option<u64>) {
    let user_fills = info.user_fills_by_time(user, start_time, end_time).await;
    // .unwrap();
    println!("User fills by time for {user} \n{:?}{SEP}", user_fills);
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

async fn recent_trades(info: &Info) {
    let coin = "ETH";

    let recent_trades = info.recent_trades(coin.to_string()).await.unwrap();
    println!("Recent trades for {coin} \n{:?}{SEP}", recent_trades);
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

async fn order_status(info: &Info, exchange: &Exchange, wallet: Arc<LocalWallet>) {
    let user = wallet.address();
    let vault_address = None;
    let cloid = Uuid::new_v4();
    let order = OrderRequest {
        asset: 4,
        is_buy: true,
        reduce_only: false,
        limit_px: parse_price(2800.0),
        sz: parse_size(0.0331, 4),
        order_type: OrderType::Limit(Limit { tif: Tif::Gtc }),
        cloid: Some(cloid),
    };

    println!("Placing order with cloid: {}{SEP}", cloid.simple());
    let response = exchange
        .place_order(wallet, vec![order], vault_address)
        .await
        .expect("Failed to place order");

    println!("Response: {:?}", response);

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let order_status = info.order_status(user, Oid::Cloid(cloid)).await.unwrap();

    println!(
        "Order status for {} \n{:?}{SEP}",
        cloid.simple(),
        order_status
    );
}

async fn sub_accounts(info: &Info, user: Address) {
    let sub_accounts = info.sub_accounts(user).await.unwrap();
    println!("Sub accounts for {user} \n{:?}{SEP}", sub_accounts);
}
