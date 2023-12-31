pub enum API {
    Info,
    Exchange,
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
    }

    pub mod testnet {
        use ethers::contract::{Eip712, EthAbiType};

        #[derive(Eip712, Clone, EthAbiType)]
        #[eip712(
            name = "Exchange",
            version = "1",
            chain_id = 421613,
            verifying_contract = "0x0000000000000000000000000000000000000000"
        )]
        pub struct UsdTransferSignPayload {
            pub destination: String,
            pub amount: String,
            pub time: u64,
        }
    }
}

pub mod info {
    pub mod request {
        use ethers::types::Address;
        use serde::Serialize;

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct CandleSnapshotRequest {
            pub coin: String,
            pub interval: String,
            pub start_time: u64,
            pub end_time: u64,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase", tag = "type")]
        pub enum Request {
            Meta,
            AllMids,
            MetaAndAssetCtxs,
            ClearinghouseState {
                user: Address,
            },
            OpenOrders {
                user: Address,
            },
            UserFills {
                user: Address,
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
            CandleSnapshot {
                req: CandleSnapshotRequest,
            },
            OrderStatus {
                user: Address,
                oid: u64,
            },
        }
    }

    pub mod response {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Asset {
            pub name: String,
            pub sz_decimals: u32,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Universe {
            pub universe: Vec<Asset>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Ctx {
            pub funding: String,
            pub open_interest: String,
            pub prev_day_px: String,
            pub day_ntl_vlm: String,
            pub premium: String,
            pub oracle_px: String,
            pub mark_px: String,
            pub mid_px: String,
            pub impact_pxs: Vec<String>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(untagged)]
        pub enum AssetContext {
            Meta(Universe),
            Ctx(Vec<Ctx>),
        }

        #[derive(Deserialize, Debug)]
        pub struct Leverage {
            #[serde(rename = "type")]
            pub type_: String,
            pub value: u32,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]

        pub struct Position {
            pub coin: String,
            pub entry_px: Option<String>,
            pub leverage: Leverage,
            pub liquidation_px: Option<String>,
            pub margin_used: String,
            pub max_trade_szs: Vec<String>,
            pub position_value: String,
            pub return_on_equity: String,
            pub szi: String,
            pub unrealized_pnl: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct AssetPosition {
            pub position: Position,
            #[serde(rename = "type")]
            pub type_: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct MarginSummary {
            pub account_value: String,
            pub total_margin_used: String,
            pub total_ntl_pos: String,
            pub total_raw_usd: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct UserState {
            pub asset_positions: Vec<AssetPosition>,
            pub margin_summary: MarginSummary,
            pub cross_margin_summary: MarginSummary,
            pub withdrawable: String,
            pub cross_maintenance_margin_used: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "UPPERCASE")]
        pub enum Side {
            B,
            A,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct OpenOrder {
            pub coin: String,
            pub limit_px: String,
            pub oid: u64,
            pub side: Side,
            pub sz: String,
            pub timestamp: u64,
        }

        #[derive(Deserialize, Debug)]
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

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Delta {
            pub coin: String,
            pub funding_rate: String,
            pub szi: String,
            #[serde(rename = "type")]
            pub type_: String,
            pub usdc: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct UserFunding {
            pub delta: Delta,
            pub hash: String,
            pub time: u64,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct FundingHistory {
            pub coin: String,
            pub funding_rate: String,
            pub premium: String,
            pub time: u64,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Level {
            pub px: String,
            pub sz: String,
            pub n: u64,
        }

        #[derive(Deserialize, Debug)]
        pub struct L2Book {
            pub coin: String,
            pub levels: Vec<Vec<Level>>,
            pub time: u64,
        }

        #[derive(Deserialize, Debug)]
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
    }
}

pub mod exchange {
    pub mod request {
        use ethers::types::{Address, Signature, H256};
        use serde::Serialize;

        #[derive(Clone, Copy, Serialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        pub enum Chain {
            Dev,
            Arbitrum,
            ArbitrumGoerli,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        pub enum Tif {
            Gtc,
            Ioc,
            Alo,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Limit {
            pub tif: Tif,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "lowercase")]
        pub enum TpSl {
            Tp,
            Sl,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Trigger {
            pub trigger_px: String,
            pub is_market: bool,
            pub tpsl: TpSl,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub enum OrderType {
            Limit(Limit),
            Trigger(Trigger),
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct OrderRequest {
            pub asset: u32,
            pub is_buy: bool,
            pub limit_px: String,
            pub sz: String,
            pub reduce_only: bool,
            pub order_type: OrderType,
        }

        impl OrderRequest {
            pub fn get_type(&self) -> (u8, u64) {
                match &self.order_type {
                    OrderType::Limit(l) => match l.tif {
                        Tif::Alo => (1, 0),
                        Tif::Gtc => (2, 0),
                        Tif::Ioc => (3, 0),
                    },
                    OrderType::Trigger(t) => match (t.is_market, &t.tpsl) {
                        (true, TpSl::Tp) => (4, t.trigger_px.parse().unwrap()),
                        (false, TpSl::Tp) => (5, t.trigger_px.parse().unwrap()),
                        (true, TpSl::Sl) => (6, t.trigger_px.parse().unwrap()),
                        (false, TpSl::Sl) => (7, t.trigger_px.parse().unwrap()),
                    },
                }
            }
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub enum Grouping {
            Na = 0,
        }
        impl Grouping {
            pub fn to_i32(&self) -> i32 {
                match self {
                    Grouping::Na => 0,
                }
            }
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct CancelRequest {
            pub oid: u64,
            pub asset: u32,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct TransferRequest {
            pub destination: String,
            pub amount: String,
            pub time: u128,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Agent {
            pub source: String,
            pub connection_id: H256,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase", tag = "type")]
        pub enum Action {
            Order {
                grouping: Grouping,
                orders: Vec<OrderRequest>,
            },
            Cancel {
                cancels: Vec<CancelRequest>,
            },
            UsdTransfer {
                chain: Chain,
                payload: TransferRequest,
            },
            Withdraw {
                usd: String,
                nonce: u128,
            },
            #[serde(rename_all = "camelCase")]
            UpdateLeverage {
                asset: u32,
                leverage: u32,
                is_cross: bool,
            },
            #[serde(rename_all = "camelCase")]
            UpdateIsolatedMargin {
                asset: u32,
                is_buy: bool,
                ntli: i64,
            },
            #[serde(rename_all = "camelCase", rename = "connect")]
            ApproveAgent {
                chain: Chain,
                agent: Agent,
                agent_address: Address,
            },
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Request {
            pub action: Action,
            pub nonce: u128,
            pub signature: Signature,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vault_address: Option<Address>,
        }
    }

    pub mod response {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        pub struct Resting {
            pub oid: u64,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Filled {
            pub oid: u64,
            pub total_sz: String,
            pub avg_px: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub enum Status {
            Resting(Resting),
            Filled(Filled),
            Error(String),
            Success,
            WaitingForFill,
            WaitingForTrigger,
        }

        #[derive(Deserialize, Debug)]
        pub struct Statuses {
            pub statuses: Vec<Status>,
        }

        #[derive(Deserialize, Debug)]
        pub struct Data {
            #[serde(rename = "type")]
            pub type_: String,
            pub data: Option<Statuses>,
        }

        #[derive(Deserialize, Debug)]
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
        use serde::Serialize;

        #[derive(Serialize, Debug, Clone)]
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

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "lowercase")]
        pub enum Method {
            Subscribe,
            Unsubscribe,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Request {
            pub method: Method,
            pub subscription: Subscription,
        }
    }

    pub mod response {
        use std::collections::HashMap;

        use ethers::types::{Address, TxHash};
        use serde::Deserialize;
        use serde_json::Value;

        use crate::types::info::response::{CandleSnapshot, Ctx, Universe, UserFill, UserState};

        #[derive(Deserialize, Debug)]
        pub struct AllMids {
            pub mids: HashMap<String, String>,
        }

        #[derive(Deserialize, Debug)]
        pub struct Notification {
            pub notification: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct LedgerUpdate {
            pub hash: TxHash,
            pub delta: Value,
            pub time: u128,
        }

        #[derive(Deserialize, Debug)]
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
            pub server_time: u128,
            pub is_vault: bool,
            pub user: Address,
        }

        #[derive(Deserialize, Debug)]
        pub struct WsTrade {
            pub coin: String,
            pub side: String,
            pub px: String,
            pub sz: String,
            pub hash: TxHash,
            pub time: u128,
        }

        #[derive(Deserialize, Debug)]
        pub struct WsLevel {
            pub px: String,
            pub sz: String,
            pub n: u64,
        }

        #[derive(Deserialize, Debug)]
        pub struct WsBook {
            pub coin: String,
            pub levels: Vec<Vec<WsLevel>>,
            pub time: u128,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct WsBasicOrder {
            pub coin: String,
            pub side: String,
            pub limit_px: String,
            pub sz: String,
            pub oid: u64,
            pub timestamp: u128,
            pub orig_sz: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct WsOrder {
            pub order: WsBasicOrder,
            pub status: String,
            pub status_timestamp: u128,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct WsUserFunding {
            pub time: u128,
            pub coin: String,
            pub usdc: String,
            pub szi: String,
            pub funding_rate: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "snake_case")]
        pub struct WsLiquidation {
            pub liq: u64,
            pub liquidator: String,
            pub liquidated_user: String,
            pub liquidated_ntl_pos: String,
            pub liquidated_account_value: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct WsNonUserCancel {
            pub oid: u64,
            pub coin: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase", untagged)]
        pub enum WsUserEvent {
            WsFill(Vec<UserFill>),
            WsUserFunding(WsUserFunding),
            WsLiquidation(WsLiquidation),
            WsNonUserCancel(Vec<WsNonUserCancel>),
        }

        #[derive(Deserialize, Debug)]
        pub struct Channel {
            pub method: String,
            pub subscription: Value,
        }

        #[derive(Deserialize, Debug)]
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
