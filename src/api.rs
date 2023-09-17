use std::{collections::HashMap, sync::Arc};

use ethers::signers::LocalWallet;

use crate::{
    client::Client, config::Config, exchange::Exchange, info::Info, types::Chain, Websocket,
};

pub enum API {
    Info,
    Exchange,
}

impl From<&API> for String {
    fn from(api: &API) -> Self {
        String::from(match api {
            API::Info => "/info",
            API::Exchange => "/exchange",
        })
    }
}

pub trait Hyperliquid {
    fn new(wallet: Arc<LocalWallet>, chain: Chain) -> Self;
    fn new_with_config(wallet: Arc<LocalWallet>, chain: Chain, config: &Config) -> Self;
}

impl Hyperliquid for Info {
    fn new(wallet: Arc<LocalWallet>, chain: Chain) -> Self {
        Self::new_with_config(wallet, chain, &Config::default())
    }
    fn new_with_config(wallet: Arc<LocalWallet>, chain: Chain, config: &Config) -> Self {
        Self {
            wallet,
            chain,
            client: Client::new(config.rest_endpoint.clone()),
        }
    }
}

impl Hyperliquid for Exchange {
    fn new(wallet: Arc<LocalWallet>, chain: Chain) -> Self {
        Self::new_with_config(wallet, chain, &Config::default())
    }
    fn new_with_config(wallet: Arc<LocalWallet>, chain: Chain, config: &Config) -> Self {
        Self {
            wallet,
            chain,
            client: Client::new(config.rest_endpoint.clone()),
        }
    }
}

impl Hyperliquid for Websocket {
    fn new(wallet: Arc<LocalWallet>, chain: Chain) -> Self {
        Self::new_with_config(wallet, chain, &Config::default())
    }
    fn new_with_config(wallet: Arc<LocalWallet>, _chain: Chain, config: &Config) -> Self {
        Self {
            wallet,
            url: config.ws_endpoint.clone(),
            stream: None,
            channels: HashMap::new(),
        }
    }
}
