use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{
    types::{
        exchange::{
            request::{CancelByCloidRequest, CancelRequest, Limit, OrderRequest, OrderType, Tif},
            response::{Response, Status, StatusType},
        },
        Chain,
    },
    Exchange, Hyperliquid,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::Dev);

    let order_type = OrderType::Limit(Limit { tif: Tif::Gtc });

    let order = OrderRequest {
        asset: 4,
        is_buy: true,
        reduce_only: false,
        limit_px: "1800".to_string(),
        sz: "0.1".to_string(),
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
        _ => {
            panic!("Order is not filled or resting, status: {:?}", status);
        }
    };

    println!("Order placed: {:?}", oid);

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Cancel order

    println!("Cancelling order with oid {oid}.");
    let cancel = CancelRequest { asset: 4, oid };

    let vault_address = None;

    let response = exchange
        .cancel_order(wallet.clone(), vec![cancel], vault_address)
        .await
        .expect("Failed to cancel order");

    println!("Response: {:?}", response);

    let cloid = Uuid::new_v4();

    println!("Placing order with cloid: {}", cloid.simple());

    let order_type = OrderType::Limit(Limit { tif: Tif::Gtc });

    let order = OrderRequest {
        asset: 4,
        is_buy: true,
        reduce_only: false,
        limit_px: "1700".to_string(),
        sz: "0.1".to_string(),
        order_type,
        cloid: Some(cloid),
    };

    let vault_address = None;

    let response = exchange
        .place_order(wallet.clone(), vec![order], vault_address)
        .await
        .expect("Failed to place order");

    let response = match response {
        Response::Ok(order) => order,
        Response::Err(error) => panic!("Failed to place order: {:?}", error),
    };

    let data = &response.data.unwrap();
    let status = match data {
        StatusType::Statuses(statuses) => &statuses[0],
        _ => {
            panic!("Failed to place order: {:?}", data);
        }
    };

    let oid = match status {
        Status::Filled(order) => order.oid,
        Status::Resting(order) => order.oid,
        _ => panic!("Order is not filled or resting"),
    };

    println!("Order placed: {:?}", oid);

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    println!("Cancelling order with cloid: {}", cloid.simple());
    let cancel = CancelByCloidRequest { asset: 4, cloid };

    let response = exchange
        .cancel_order_by_cloid(wallet.clone(), vec![cancel], vault_address)
        .await
        .expect("Failed to cancel order");

    println!("Response: {:?}", response);
}
