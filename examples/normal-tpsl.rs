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

    let normal = OrderRequest {
        asset,
        is_buy: true,
        reduce_only: false,
        limit_px: parse_price(2800.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type: OrderType::Limit(Limit { tif: Tif::Gtc }),
        cloid: None,
    };

    let tp = OrderRequest {
        asset,
        is_buy: false,
        reduce_only: true,
        limit_px: parse_price(2810.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type: OrderType::Trigger(Trigger {
            is_market: true,
            trigger_px: parse_price(2810.0),
            tpsl: TpSl::Tp,
        }),
        cloid: None,
    };

    let sl = OrderRequest {
        asset,
        is_buy: false,
        reduce_only: true,
        limit_px: parse_price(2750.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type: OrderType::Trigger(Trigger {
            is_market: true,
            trigger_px: parse_price(2750.0),
            tpsl: TpSl::Tp,
        }),
        cloid: None,
    };

    let vault_address = None;

    println!("Placing normal tpsl order...");
    let response = exchange
        .normal_tpsl(wallet.clone(), vec![normal, tp, sl], vault_address)
        .await
        .expect("Failed to place order");

    println!("Response: {:?}", response);

    println!("-----------------");
}
