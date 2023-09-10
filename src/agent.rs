pub mod mainnet {
    use ethers::{
        contract::{Eip712, EthAbiType},
        types::H256,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Eip712, Clone, EthAbiType, Serialize, Deserialize)]
    #[eip712(
        name = "Exchange",
        version = "1",
        chain_id = 42161,
        verifying_contract = "0x0000000000000000000000000000000000000000"
    )]
    #[serde(rename_all = "camelCase")]
    pub struct Agent {
        pub source: String,
        pub connection_id: H256,
    }
}

pub mod testnet {
    use ethers::{
        contract::{Eip712, EthAbiType},
        types::H256,
    };

    #[derive(Eip712, Clone, EthAbiType)]
    #[eip712(
        name = "Exchange",
        version = "1",
        chain_id = 421613,
        verifying_contract = "0x0000000000000000000000000000000000000000"
    )]
    pub struct Agent {
        pub source: String,
        pub connection_id: H256,
    }
}

pub mod l1 {
    use ethers::{
        contract::{Eip712, EthAbiType},
        types::H256,
    };

    #[derive(Eip712, Clone, EthAbiType)]
    #[eip712(
        name = "Exchange",
        version = "1",
        chain_id = 1337,
        verifying_contract = "0x0000000000000000000000000000000000000000"
    )]
    pub struct Agent {
        pub source: String,
        pub connection_id: H256,
    }
}
