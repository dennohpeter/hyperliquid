use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{
    types::{
        exchange::request::{Limit, OrderRequest, OrderType, Tif, TpSl, Trigger},
        Chain,
    },
    utils::{parse_price, parse_size},
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

    println!("Response: {:?}", response);

    println!("-----------------");
    println!("Placing an order with cloid...");

    let order_type = OrderType::Limit(Limit { tif: Tif::Gtc });

    let cloid = Uuid::new_v4();

    let order = OrderRequest {
        asset,
        is_buy: true,
        reduce_only: false,
        limit_px: parse_price(2800.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type,
        cloid: Some(cloid),
    };

    let response = exchange
        .place_order(wallet.clone(), vec![order], vault_address)
        .await
        .expect("Failed to place order");

    println!("Response: {:?}", response);

    println!("-----------------");
    println!("Placing a trigger order with tpsl...");

    let order_type = OrderType::Trigger(Trigger {
        is_market: false,
        trigger_px: parse_price(2800.0),
        tpsl: TpSl::Tp,
    });

    let order = OrderRequest {
        asset,
        is_buy: true,
        reduce_only: false,
        limit_px: parse_price(2800.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type,
        cloid: Some(cloid),
    };

    let response = exchange
        .place_order(wallet.clone(), vec![order], vault_address)
        .await
        .expect("Failed to place order");

    println!("Response: {:?}", response);
}
