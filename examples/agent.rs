/*
* Example assumes you already have a position on ETH so you can update margin
*/

use std::sync::Arc;

use ethers::{
    core::rand::thread_rng,
    signers::{LocalWallet, Signer},
};
use hyperliquid::{
    types::{
        exchange::request::{Limit, OrderRequest, OrderType, Tif},
        Chain,
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

    let exchange: Exchange = Hyperliquid::new(Chain::ArbitrumTestnet);

    // Create a new wallet with the agent. This agent can't transfer or withdraw funds
    // but can place orders.

    let agent = Arc::new(LocalWallet::new(&mut thread_rng()));

    let agent_address = agent.address();

    println!("Agent address: {:?}", agent_address);

    let res = exchange
        .approve_agent(wallet.clone(), agent_address, Some("WETH".to_string()))
        .await
        .unwrap();

    println!("Response: {:?}", res);

    // place order with agent
    let order_type = OrderType::Limit(Limit { tif: Tif::Gtc });
    let order = OrderRequest {
        asset: 4,
        is_buy: true,
        reduce_only: false,
        limit_px: "1700".to_string(),
        sz: "0.1".to_string(),
        order_type,
        cloid: None,
    };
    let vault_address = None;

    println!("Placing order with agent...");

    let response = exchange
        .place_order(agent.clone(), vec![order], vault_address)
        .await
        .expect("Failed to place order");

    println!("Response: {:?}", response);
}
