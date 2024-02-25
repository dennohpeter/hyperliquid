use std::collections::HashMap;

use crate::{
    client::Client,
    config::Config,
    exchange::Exchange,
    info::Info,
    types::{Chain, API},
    Websocket,
};

impl From<&API> for String {
    fn from(api: &API) -> Self {
        String::from(match api {
            API::Info => "/info",
            API::Exchange => "/exchange",
        })
    }
}

pub trait Hyperliquid {
    fn new(chain: Chain) -> Self;
    fn new_with_config(chain: Chain, config: &Config) -> Self;
}

impl Hyperliquid for Info {
    fn new(chain: Chain) -> Self {
        let config = match chain {
            Chain::Arbitrum => Config::mainnet(),
            Chain::ArbitrumGoerli | Chain::ArbitrumTestnet => Config::testnet(),
            _ => Config::default(),
        };
        Self::new_with_config(chain, &config)
    }
    fn new_with_config(chain: Chain, config: &Config) -> Self {
        Self {
            chain,
            client: Client::new(config.rest_endpoint.clone()),
        }
    }
}

impl Hyperliquid for Exchange {
    fn new(chain: Chain) -> Self {
        let config = match chain {
            Chain::Arbitrum => Config::mainnet(),
            Chain::ArbitrumGoerli | Chain::ArbitrumTestnet => Config::testnet(),
            _ => Config::default(),
        };
        Self::new_with_config(chain, &config)
    }
    fn new_with_config(chain: Chain, config: &Config) -> Self {
        Self {
            chain,
            client: Client::new(config.rest_endpoint.clone()),
        }
    }
}

impl Hyperliquid for Websocket {
    fn new(chain: Chain) -> Self {
        let config = match chain {
            Chain::Arbitrum => Config::mainnet(),
            Chain::ArbitrumGoerli | Chain::ArbitrumTestnet => Config::testnet(),
            _ => Config::default(),
        };
        Self::new_with_config(chain, &config)
    }
    fn new_with_config(_chain: Chain, config: &Config) -> Self {
        Self {
            url: config.ws_endpoint.clone(),
            stream: None,
            channels: HashMap::new(),
        }
    }
}
