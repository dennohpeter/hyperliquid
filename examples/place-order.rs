use std::sync::Arc;

use ethers::{
    signers::LocalWallet,
    types::{Chain, H128},
};
use hyperliquid::{
    types::exchange::request::{Limit, OrderRequest, OrderType, Tif, TpSl, Trigger},
    utils::{parse_price, parse_size},
    Exchange, Hyperliquid,
};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "9dd680334f79f0e6c82da3b20a1942c4a9a2e14d1eb32342012bf468c52bd85f"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::Dev);

    let asset = 4;
    let sz_decimals = 4;

    let order_type = OrderType::Trigger(Trigger {
        is_market: false,
        trigger_px: parse_price(2800.0),
        tpsl: TpSl::Tp,
    });
    //  OrderType::Limit(Limit { tif: Tif::Gtc });

    let order = OrderRequest {
        asset,
        is_buy: true,
        reduce_only: false,
        limit_px: parse_price(2800.0),
        sz: parse_size(0.0331, sz_decimals),
        order_type,
        cloid: Some(H128::random()),
    };

    let vault_address = None;

    println!("Placing order...");
    let response = exchange
        .place_order(wallet.clone(), vec![order], vault_address)
        .await
        .expect("Failed to place order");

    println!("Response: {:?}", response);
}
