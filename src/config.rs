pub struct Config {
    pub rest_endpoint: String,
    pub ws_endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::testnet()
    }
}

impl Config {
    pub fn mainnet() -> Self {
        Self {
            rest_endpoint: "https://api.hyperliquid.xyz".to_string(),
            ws_endpoint: "wss://api.hyperliquid.xyz/ws".to_string(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            rest_endpoint: "https://api.hyperliquid-testnet.xyz".to_string(),
            ws_endpoint: "wss://api.hyperliquid-testnet.xyz/ws".to_string(),
        }
    }

    pub fn local() -> Self {
        Self {
            rest_endpoint: "http://localhost:3001".to_string(),
            ws_endpoint: "ws://localhost:3001/ws".to_string(),
        }
    }

    pub fn set_rest_endpoint(&mut self, endpoint: String) {
        self.rest_endpoint = endpoint;
    }

    pub fn set_ws_endpoint(&mut self, endpoint: String) {
        self.ws_endpoint = endpoint;
    }
}
