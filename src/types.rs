use crate::utils::as_hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum Chain {
    Dev = 1337,

    Arbitrum = 42161,
    ArbitrumTestnet = 421611,
    ArbitrumGoerli = 421613,
    ArbitrumSepolia = 421614,
    ArbitrumNova = 42170,
}

impl ToString for Chain {
    fn to_string(&self) -> String {
        String::from(match self {
            Chain::Dev => "Dev",
            Chain::Arbitrum => "Arbitrum",
            Chain::ArbitrumTestnet => "ArbitrumTestnet",
            Chain::ArbitrumGoerli => "ArbitrumGoerli",
            Chain::ArbitrumSepolia => "ArbitrumSepolia",
            Chain::ArbitrumNova => "ArbitrumNova",
        })
    }
}

pub enum API {
    Info,
    Exchange,
}

pub type Cloid = Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Oid {
    Order(u64),
    #[serde(serialize_with = "as_hex")]
    Cloid(Cloid),
}
pub mod agent {
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
            chain_id = 421614,
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
}

pub mod usd_transfer {
    pub mod mainnet {
        use ethers::contract::{Eip712, EthAbiType};

        #[derive(Eip712, Clone, EthAbiType)]
        #[eip712(
            name = "Exchange",
            version = "1",
            chain_id = 42161,
            verifying_contract = "0x0000000000000000000000000000000000000000"
        )]
        pub struct UsdTransferSignPayload {
            pub destination: String,
            pub amount: String,
            pub time: u64,
        }

        #[derive(Eip712, Clone, EthAbiType)]
        #[eip712(
            name = "Exchange",
            version = "1",
            chain_id = 42161,
            verifying_contract = "0x0000000000000000000000000000000000000000"
        )]
        pub struct WithdrawFromBridge2SignPayload {
            pub destination: String,
            pub usd: String,
            pub time: u64,
        }
    }

    pub mod testnet {
        use ethers::contract::{Eip712, EthAbiType};

        #[derive(Eip712, Clone, EthAbiType)]
        #[eip712(
            name = "Exchange",
            version = "1",
            chain_id = 421614,
            verifying_contract = "0x0000000000000000000000000000000000000000"
        )]
        pub struct UsdTransferSignPayload {
            pub destination: String,
            pub amount: String,
            pub time: u64,
        }

        #[derive(Eip712, Clone, EthAbiType)]
        #[eip712(
            name = "Exchange",
            version = "1",
            chain_id = 421614,
            verifying_contract = "0x0000000000000000000000000000000000000000"
        )]
        pub struct WithdrawFromBridge2SignPayload {
            pub destination: String,
            pub usd: String,
            pub time: u64,
        }
    }
}

pub mod info {
    pub mod request {
        use ethers::types::Address;
        use serde::{Deserialize, Serialize};

        use crate::types::Oid;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CandleSnapshotRequest {
            pub coin: String,
            pub interval: String,
            pub start_time: u64,
            pub end_time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", tag = "type")]
        pub enum Request {
            Meta,
            AllMids,
            MetaAndAssetCtxs,
            ClearinghouseState {
                user: Address,
            },
            BatchClearinghouseStates {
                users: Vec<Address>,
            },
            OpenOrders {
                user: Address,
            },

            FrontendOpenOrders {
                user: Address,
            },
            UserFills {
                user: Address,
            },
            #[serde(rename_all = "camelCase")]
            UserFillsByTime {
                user: Address,
                start_time: u64,
                #[serde(skip_serializing_if = "Option::is_none")]
                end_time: Option<u64>,
            },
            #[serde(rename_all = "camelCase")]
            UserFunding {
                user: Address,
                start_time: u64,
                end_time: Option<u64>,
            },
            #[serde(rename_all = "camelCase")]
            FundingHistory {
                coin: String,
                start_time: u64,
                end_time: Option<u64>,
            },
            L2Book {
                coin: String,
            },
            RecentTrades {
                coin: String,
            },
            CandleSnapshot {
                req: CandleSnapshotRequest,
            },
            OrderStatus {
                user: Address,
                oid: Oid,
            },
            SubAccounts {
                user: Address,
            },
        }
    }

    pub mod response {
        use ethers::types::Address;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Asset {
            pub name: String,
            pub sz_decimals: u32,
            pub max_leverage: u32,
            pub only_isolated: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Universe {
            pub universe: Vec<Asset>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum ImpactPx {
            String(String),
            StringArray(Vec<String>),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Ctx {
            pub funding: String,
            pub open_interest: String,
            pub prev_day_px: String,
            pub day_ntl_vlm: String,
            pub premium: Option<String>,
            pub oracle_px: String,
            pub mark_px: String,
            pub mid_px: Option<String>,
            pub impact_pxs: Option<ImpactPx>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum AssetContext {
            Meta(Universe),
            Ctx(Vec<Ctx>),
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Leverage {
            #[serde(rename = "type")]
            pub type_: String,
            pub value: u32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]

        pub struct Position {
            pub coin: String,
            pub entry_px: Option<String>,
            pub leverage: Leverage,
            pub liquidation_px: Option<String>,
            pub margin_used: String,
            pub position_value: String,
            pub return_on_equity: String,
            pub szi: String,
            pub unrealized_pnl: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AssetPosition {
            pub position: Position,
            #[serde(rename = "type")]
            pub type_: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct MarginSummary {
            pub account_value: String,
            pub total_margin_used: String,
            pub total_ntl_pos: String,
            pub total_raw_usd: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UserState {
            pub asset_positions: Vec<AssetPosition>,
            pub margin_summary: MarginSummary,
            pub cross_margin_summary: MarginSummary,
            pub withdrawable: String,
            pub time: u64,
            pub cross_maintenance_margin_used: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "UPPERCASE")]
        pub enum Side {
            B,
            A,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct OpenOrder {
            pub coin: String,
            pub limit_px: String,
            pub oid: u64,
            pub side: Side,
            pub sz: String,
            pub timestamp: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FrontendOpenOrders {
            pub coin: String,
            pub is_position_tpsl: bool,
            pub is_trigger: bool,
            pub limit_px: String,
            pub oid: u64,
            pub order_type: String,
            pub orig_sz: String,
            pub reduce_only: bool,
            pub side: Side,
            pub sz: String,
            pub timestamp: u64,
            pub trigger_condition: String,
            pub trigger_px: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UserFill {
            pub coin: String,
            pub px: String,
            pub sz: String,
            pub side: Side,
            pub time: u64,
            pub start_position: String,
            pub dir: String,
            pub closed_pnl: String,
            pub hash: String,
            pub oid: u64,
            pub crossed: bool,
            pub fee: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Delta {
            pub coin: String,
            pub funding_rate: String,
            pub szi: String,
            #[serde(rename = "type")]
            pub type_: String,
            pub usdc: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UserFunding {
            pub delta: Delta,
            pub hash: String,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct FundingHistory {
            pub coin: String,
            pub funding_rate: String,
            pub premium: String,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Level {
            pub px: String,
            pub sz: String,
            pub n: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct L2Book {
            pub coin: String,
            pub levels: Vec<Vec<Level>>,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct RecentTrades {
            pub coin: String,
            pub side: Side,
            pub px: String,
            pub sz: String,
            pub hash: String,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CandleSnapshot {
            #[serde(rename = "T")]
            pub t_: u64,
            pub c: String,
            pub h: String,
            pub i: String,
            pub l: String,
            pub n: u64,
            pub o: String,
            pub s: String,
            pub t: u64,
            pub v: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct OrderInfo {
            pub children: Vec<Option<serde_json::Value>>,
            pub cloid: Option<String>,
            pub coin: String,
            pub is_position_tpsl: bool,
            pub is_trigger: bool,
            pub limit_px: String,
            pub oid: i64,
            pub order_type: String,
            pub orig_sz: String,
            pub reduce_only: bool,
            pub side: String,
            pub sz: String,
            pub tif: Option<String>,
            pub timestamp: i64,
            pub trigger_condition: String,
            pub trigger_px: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Order {
            pub order: OrderInfo,
            pub status: String,
            pub status_timestamp: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct OrderStatus {
            pub order: Option<Order>,
            pub status: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SubAccount {
            pub clearinghouse_state: UserState,
            pub master: Address,
            pub name: String,
            pub sub_account_user: Address,
        }
    }
}

pub mod exchange {
    pub mod request {
        use ethers::{
            types::{Address, Signature, H256},
            utils::keccak256,
        };
        use serde::{Deserialize, Serialize};

        use crate::{
            types::{Chain, Cloid},
            utils::{as_hex, as_hex_option},
            Error, Result,
        };

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub enum Tif {
            Gtc,
            Ioc,
            Alo,
            FrontendMarket,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Limit {
            pub tif: Tif,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum TpSl {
            Tp,
            Sl,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Trigger {
            pub is_market: bool,
            pub trigger_px: String,
            pub tpsl: TpSl,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum OrderType {
            Limit(Limit),
            Trigger(Trigger),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct OrderRequest {
            #[serde(rename = "a", alias = "asset")]
            pub asset: u32,
            #[serde(rename = "b", alias = "isBuy")]
            pub is_buy: bool,
            #[serde(rename = "p", alias = "limitPx")]
            pub limit_px: String,
            #[serde(rename = "s", alias = "sz")]
            pub sz: String,
            #[serde(rename = "r", alias = "reduceOnly", default)]
            pub reduce_only: bool,
            #[serde(rename = "t", alias = "orderType")]
            pub order_type: OrderType,
            #[serde(
                rename = "c",
                alias = "cloid",
                serialize_with = "as_hex_option",
                skip_serializing_if = "Option::is_none"
            )]
            pub cloid: Option<Cloid>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum Grouping {
            Na,
            NormalTpsl,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CancelRequest {
            #[serde(rename = "a", alias = "asset")]
            pub asset: u32,
            #[serde(rename = "o", alias = "oid")]
            pub oid: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CancelByCloidRequest {
            pub asset: u32,
            #[serde(serialize_with = "as_hex")]
            pub cloid: Cloid,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct ModifyRequest {
            pub oid: u64,
            pub order: OrderRequest,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TwapRequest {
            #[serde(rename = "a", alias = "asset")]
            pub asset: u32,
            #[serde(rename = "b", alias = "isBuy")]
            pub is_buy: bool,
            #[serde(rename = "s", alias = "sz")]
            pub sz: String,
            #[serde(rename = "r", alias = "reduceOnly", default)]
            pub reduce_only: bool,
            /// Running Time (5m - 24h)
            #[serde(rename = "m", alias = "duration")]
            pub duration: u64,
            /// if set to true, the size of each sub-trade will be automatically adjusted
            /// within a certain range, typically upto to 20% higher or lower than the original trade size
            #[serde(rename = "t", alias = "randomize")]
            pub randomize: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TransferRequest {
            pub destination: String,
            pub amount: String,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WithdrawalRequest {
            pub destination: String,
            pub usd: String,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Agent {
            pub source: String,
            pub connection_id: H256,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", tag = "type")]
        pub enum Action {
            Order {
                orders: Vec<OrderRequest>,
                grouping: Grouping,
            },
            Cancel {
                cancels: Vec<CancelRequest>,
            },
            CancelByCloid {
                cancels: Vec<CancelByCloidRequest>,
            },

            Modify(ModifyRequest),

            BatchModify {
                modifies: Vec<ModifyRequest>,
            },
            TwapOrder {
                twap: TwapRequest,
            },
            UsdTransfer {
                chain: Chain,
                payload: TransferRequest,
            },
            Withdraw {
                usd: String,
                nonce: u64,
            },
            Withdraw2 {
                chain: Chain,
                payload: WithdrawalRequest,
            },
            #[serde(rename_all = "camelCase")]
            UpdateLeverage {
                asset: u32,
                is_cross: bool,
                leverage: u32,
            },
            #[serde(rename_all = "camelCase")]
            UpdateIsolatedMargin {
                asset: u32,
                is_buy: bool,
                ntli: i64,
            },
            #[serde(rename_all = "camelCase")]
            Connect {
                chain: Chain,
                agent: Agent,
                agent_address: Address,
                #[serde(skip_serializing_if = "Option::is_none")]
                extra_agent_name: Option<String>,
            },
            CreateSubAccount {
                name: String,
            },
            #[serde(rename_all = "camelCase")]
            SubAccountTransfer {
                sub_account_user: String,
                is_deposit: bool,
                usd: String,
            },
            SetReferrer {
                code: String,
            },
            ScheduleCancel {
                time: u64,
            },
        }

        impl Action {
            /// create connection id for agent
            pub fn connection_id(
                &self,
                vault_address: Option<Address>,
                nonce: u64,
            ) -> Result<H256> {
                let mut encoded = rmp_serde::to_vec_named(self)
                    .map_err(|e| Error::RmpSerdeError(e.to_string()))?;

                encoded.extend((nonce).to_be_bytes());

                if let Some(address) = vault_address {
                    encoded.push(1);
                    encoded.extend(address.to_fixed_bytes());
                } else {
                    encoded.push(0)
                }

                Ok(keccak256(encoded).into())
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Request {
            pub action: Action,
            pub nonce: u64,
            pub signature: Signature,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vault_address: Option<Address>,
        }
    }

    pub mod response {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Resting {
            pub oid: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Filled {
            pub oid: u64,
            pub total_sz: String,
            pub avg_px: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TwapId {
            pub twap_id: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum Status {
            Resting(Resting),
            Filled(Filled),
            Error(String),
            Success,
            WaitingForFill,
            WaitingForTrigger,
            Running(TwapId),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum StatusType {
            Statuses(Vec<Status>),
            Status(Status),
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Data {
            #[serde(rename = "type")]
            pub type_: String,
            pub data: Option<StatusType>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", tag = "status", content = "response")]
        pub enum Response {
            Ok(Data),
            Err(String),
        }
    }
}

pub mod websocket {
    pub mod request {
        use ethers::types::Address;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone)]
        #[serde(rename_all = "camelCase", tag = "type")]
        pub enum Subscription {
            AllMids,
            Notification { user: Address },
            OrderUpdates { user: Address },
            User { user: Address },
            WebData { user: Address },
            L2Book { coin: String },
            Trades { coin: String },
            Candle { coin: String, interval: String },
        }

        #[derive(Clone)]
        pub struct Channel {
            pub id: u64,
            pub sub: Subscription,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum Method {
            Subscribe,
            Unsubscribe,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Request {
            pub method: Method,
            pub subscription: Subscription,
        }
    }

    pub mod response {
        use std::collections::HashMap;

        use ethers::types::{Address, TxHash};
        use serde::{Deserialize, Serialize};
        use serde_json::Value;

        use crate::types::info::response::{CandleSnapshot, Ctx, Universe, UserFill, UserState};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AllMids {
            pub mids: HashMap<String, String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Notification {
            pub notification: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LedgerUpdate {
            pub hash: TxHash,
            pub delta: Value,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WebData {
            pub user_state: UserState,
            pub lending_vaults: Option<Vec<Value>>,
            pub total_vault_equity: String,
            pub open_orders: Vec<Value>,
            pub fills: Vec<Value>,
            pub whitelisted: bool,
            pub ledger_updates: Vec<LedgerUpdate>,
            pub agent_address: Address,
            pub pending_withdraws: Option<Vec<Value>>,
            pub cum_ledger: String,
            pub meta: Universe,
            pub asset_contexts: Option<Vec<Ctx>>,
            pub order_history: Vec<Value>,
            pub server_time: u64,
            pub is_vault: bool,
            pub user: Address,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WsTrade {
            pub coin: String,
            pub side: String,
            pub px: String,
            pub sz: String,
            pub hash: TxHash,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WsLevel {
            pub px: String,
            pub sz: String,
            pub n: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WsBook {
            pub coin: String,
            pub levels: Vec<Vec<WsLevel>>,
            pub time: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WsBasicOrder {
            pub coin: String,
            pub side: String,
            pub limit_px: String,
            pub sz: String,
            pub oid: u64,
            pub timestamp: u64,
            pub orig_sz: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WsOrder {
            pub order: WsBasicOrder,
            pub status: String,
            pub status_timestamp: u64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WsUserFunding {
            pub time: u64,
            pub coin: String,
            pub usdc: String,
            pub szi: String,
            pub funding_rate: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub struct WsLiquidation {
            pub liq: u64,
            pub liquidator: String,
            pub liquidated_user: String,
            pub liquidated_ntl_pos: String,
            pub liquidated_account_value: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct WsNonUserCancel {
            pub oid: u64,
            pub coin: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", untagged)]
        pub enum WsUserEvent {
            WsFill(Vec<UserFill>),
            WsUserFunding(WsUserFunding),
            WsLiquidation(WsLiquidation),
            WsNonUserCancel(Vec<WsNonUserCancel>),
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Channel {
            pub method: String,
            pub subscription: Value,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", tag = "channel", content = "data")]
        pub enum Response {
            AllMids(AllMids),
            Notification(Notification),
            WebData(WebData),
            Candle(CandleSnapshot),
            L2Book(WsBook),
            Trades(Vec<WsTrade>),
            OrderUpdates(WsOrder),
            User(WsUserEvent),
            SubscriptionResponse(Channel),
        }
    }
}
