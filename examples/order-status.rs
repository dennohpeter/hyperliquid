use std::sync::Arc;

use ethers::signers::{LocalWallet, Signer};
use hyperliquid::{
    types::{
        exchange::{
            request::{Limit, OrderRequest, OrderType, Tif},
            response::{Response, Status, StatusType},
        },
        Chain, Oid,
    },
    utils::{parse_price, parse_size},
    Exchange, Hyperliquid, Info,
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
    let info: Info = Hyperliquid::new(Chain::Dev);

    let asset = 4;
    let sz_decimals = 4;

    let order_type = OrderType::Limit(Limit { tif: Tif::Gtc });

    let order = OrderRequest {
        asset,
        is_buy: true,
        reduce_only: false,
        limit_px: parse_price(2800.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type,
        cloid: None,
    };

    let vault_address = None;

    println!("Placing order...");
    let response = exchange
        .place_order(wallet.clone(), vec![order], vault_address)
        .await
        .expect("Failed to place order");

    let response = match response {
        Response::Ok(order) => order,
        Response::Err(error) => panic!("Failed to place order: {:?}", error),
    };

    println!("Response: {:?}", response.data);

    let status_type = &response.data.unwrap();

    let status = match status_type {
        StatusType::Statuses(statuses) => &statuses[0],
        _ => {
            panic!("Failed to place order: {:?}", status_type);
        }
    };

    let oid = match status {
        Status::Filled(order) => order.oid,
        Status::Resting(order) => order.oid,
        _ => panic!("Order is not filled or resting"),
    };

    println!("-----------------");

    println!("Fetching order {} status...", oid);

    let status = info
        .order_status(wallet.address(), Oid::Order(oid))
        .await
        .expect("Failed to fetch order status");

    println!("Order status: {:#?}", status.order);
}
