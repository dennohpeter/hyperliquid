use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{
    types::exchange::{
        request::{CancelRequest, Chain, Limit, OrderRequest, OrderType, Tif},
        response::{Response, Status},
    },
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

    let order_type = OrderType::Limit(Limit { tif: Tif::Gtc });

    let order = OrderRequest {
        asset: 4,
        is_buy: true,
        reduce_only: false,
        limit_px: "1700".to_string(),
        sz: "0.1".to_string(),
        order_type,
    };

    let vault_address = None;

    println!("Placing order...");
    let response = exchange
        .place_order(wallet.clone(), order, vault_address)
        .await
        .expect("Failed to place order");

    let response = match response {
        Response::Ok(order) => order,
        Response::Err(error) => panic!("Failed to place order: {:?}", error),
    };

    let status = &response.data.unwrap().statuses[0];

    let oid = match status {
        Status::Filled(order) => order.oid,
        Status::Resting(order) => order.oid,
        _ => panic!("Order is not filled or resting"),
    };

    println!("Order placed: {:?}", oid);

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    // Cancel order

    println!("Cancelling order...");
    let cancel = CancelRequest { asset: 4, oid };

    let vault_address = None;

    let response = exchange
        .cancel_order(wallet.clone(), cancel, vault_address)
        .await
        .expect("Failed to cancel order");

    println!("Response: {:?}", response);
}
