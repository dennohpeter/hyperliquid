use ethers::signers::LocalWallet;

use crate::{client::Client, config::Config, exchange::Exchange, info::Info, types::Chain};

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
    fn new(wallet: LocalWallet, chain: Chain) -> Self;
    fn new_with_config(wallet: LocalWallet, chain: Chain, config: &Config) -> Self;
}

impl Hyperliquid for Info {
    fn new(wallet: LocalWallet, chain: Chain) -> Self {
        Self::new_with_config(wallet, chain, &Config::default())
    }
    fn new_with_config(wallet: LocalWallet, chain: Chain, config: &Config) -> Self {
        Self {
            wallet,
            chain,
            client: Client::new(config.rest_endpoint.clone()),
        }
    }
}

impl Hyperliquid for Exchange {
    fn new(wallet: LocalWallet, chain: Chain) -> Self {
        Self::new_with_config(wallet, chain, &Config::default())
    }
    fn new_with_config(wallet: LocalWallet, chain: Chain, config: &Config) -> Self {
        Self {
            wallet,
            chain,
            client: Client::new(config.rest_endpoint.clone()),
        }
    }
}
