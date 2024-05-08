use std::collections::HashMap;

use ethers::types::Address;

use crate::{
    client::Client,
    error::Result,
    types::{
        info::{
            request::{CandleSnapshotRequest, Request},
            response::{
                AssetContext, CandleSnapshot, FrontendOpenOrders, FundingHistory, L2Book,
                OpenOrder, OrderStatus, RecentTrades, SpotMeta, SpotMetaAndAssetCtxs, SubAccount,
                Universe, UserFill, UserFunding, UserSpotState, UserState,
            },
        },
        Chain, Oid, API,
    },
};

/// Endpoint to fetch information about the exchange and specific users.
pub struct Info {
    pub client: Client,
    pub chain: Chain,
}

impl Info {
    /// Retrieve exchange metadata
    pub async fn metadata(&self) -> Result<Universe> {
        self.client.post(&API::Info, &Request::Meta).await
    }

    /// Retrieve all mids for all actively traded coins
    pub async fn mids(&self) -> Result<HashMap<String, String>> {
        self.client.post(&API::Info, &Request::AllMids).await
    }

    /// Retrieve asset contexts i.e mark price, current funding, open interest, etc
    pub async fn contexts(&self) -> Result<Vec<AssetContext>> {
        self.client
            .post(&API::Info, &Request::MetaAndAssetCtxs)
            .await
    }

    /// Retrieve a user's state to see user's open positions and margin summary
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    pub async fn user_state(&self, user: Address) -> Result<UserState> {
        self.client
            .post(&API::Info, &Request::ClearinghouseState { user })
            .await
    }

    /// Retrieve a user's state to see user's open positions and margin summary in batch
    ///
    /// # Arguments
    /// * `users` - A list of user addresses in 42-character hexadecimal format
    pub async fn user_states(&self, users: Vec<Address>) -> Result<Vec<UserState>> {
        self.client
            .post(&API::Info, &Request::BatchClearinghouseStates { users })
            .await
    }

    /// Retrieve a user's open orders
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    pub async fn open_orders(&self, user: Address) -> Result<Vec<OpenOrder>> {
        self.client
            .post(&API::Info, &Request::OpenOrders { user })
            .await
    }

    /// Retrieve a user's open orders with additional frontend info.
    /// This is useful for displaying orders in a UI
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    pub async fn frontend_open_orders(&self, user: Address) -> Result<Vec<FrontendOpenOrders>> {
        self.client
            .post(&API::Info, &Request::FrontendOpenOrders { user })
            .await
    }

    /// Retrieve a user's Userfills
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    pub async fn user_fills(&self, user: Address) -> Result<Vec<UserFill>> {
        self.client
            .post(&API::Info, &Request::UserFills { user })
            .await
    }

    /// Retrieve a user's fills by time
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive. If `None`, it will default to the current time
    ///
    /// # Note
    /// * Number of fills is limited to 2000
    pub async fn user_fills_by_time(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<UserFill>> {
        self.client
            .post(
                &API::Info,
                &Request::UserFillsByTime {
                    user,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve a user's funding history
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive. If `None`, it will default to the current time
    pub async fn user_funding(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<UserFunding>> {
        self.client
            .post(
                &API::Info,
                &Request::UserFunding {
                    user,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve historical funding rates for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve funding history for e.g `BTC`, `ETH`, etc
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive. If `None`, it will default to the current time
    pub async fn funding_history(
        &self,
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<FundingHistory>> {
        self.client
            .post(
                &API::Info,
                &Request::FundingHistory {
                    coin,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve the L2 order book for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve the L2 order book for e.g `BTC`, `ETH`, etc
    pub async fn l2_book(&self, coin: String) -> Result<L2Book> {
        self.client
            .post(&API::Info, &Request::L2Book { coin })
            .await
    }

    /// Retrieve the recent trades for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve the recent trades for
    pub async fn recent_trades(&self, coin: String) -> Result<Vec<RecentTrades>> {
        self.client
            .post(&API::Info, &Request::RecentTrades { coin })
            .await
    }

    /// Retrieve candle snapshot for a coin
    ///
    /// # Arguments
    /// * `coin` - The coin to retrieve the candle snapshot for e.g `BTC`, `ETH`, etc
    /// * `interval` - The interval to retrieve the candle snapshot for
    /// * `start_time` - Start time in milliseconds, inclusive
    /// * `end_time` - End time in milliseconds, inclusive.
    pub async fn candle_snapshot(
        &self,
        coin: String,
        interval: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<CandleSnapshot>> {
        self.client
            .post(
                &API::Info,
                &Request::CandleSnapshot {
                    req: CandleSnapshotRequest {
                        coin,
                        interval,
                        start_time,
                        end_time,
                    },
                },
            )
            .await
    }

    /// Query the status of an order by `oid` or `cloid`
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    /// * `oid` - The order id either u64 representing the order id or 16-byte hex string representing the client order id
    pub async fn order_status(&self, user: Address, oid: Oid) -> Result<OrderStatus> {
        self.client
            .post(&API::Info, &Request::OrderStatus { user, oid })
            .await
    }

    /// Query user sub-accounts
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    pub async fn sub_accounts(&self, user: Address) -> Result<Option<Vec<SubAccount>>> {
        self.client
            .post(&API::Info, &Request::SubAccounts { user })
            .await
    }
}

impl Info {
    /// Retrieve spot metadata
    pub async fn spot_meta(&self) -> Result<SpotMeta> {
        self.client.post(&API::Info, &Request::SpotMeta).await
    }

    /// Retrieve spot asset contexts
    pub async fn spot_meta_and_asset_ctxs(&self) -> Result<Vec<SpotMetaAndAssetCtxs>> {
        self.client
            .post(&API::Info, &Request::SpotMetaAndAssetCtxs)
            .await
    }

    /// Retrieve a user's token balances
    ///
    /// # Arguments
    /// * `user` - The user's address in 42-character hexadecimal format; e.g. `0x0000000000000000000000000000000000000000`
    pub async fn spot_clearinghouse_state(&self, user: Address) -> Result<UserSpotState> {
        self.client
            .post(&API::Info, &Request::SpotClearinghouseState { user })
            .await
    }
}
