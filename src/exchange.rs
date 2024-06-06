use std::{sync::Arc, time::SystemTime};

use ethers::{
    signers::{LocalWallet, Signer},
    types::{Address, Signature, H256},
    utils::to_checksum,
};

use crate::{
    client::Client,
    error::Result,
    types::{
        agent::l1,
        exchange::{
            request::{
                Action, ApproveAgent, CancelByCloidRequest, CancelRequest, Grouping, ModifyRequest,
                OrderRequest, Request, TwapRequest, UsdSend, Withdraw3,
            },
            response::Response,
        },
        Chain, HyperliquidChain, API,
    },
    Error,
};

/// Endpoint to interact with and trade on the Hyperliquid chain.
pub struct Exchange {
    pub client: Client,
    pub chain: Chain,
}

impl Exchange {
    /// Place an order
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `orders` - The orders to place
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    ///  e.g. `0x0000000000000000000000000000000000000000`
    ///
    ///  # Note
    /// * `cloid` in argument `order` is an optional 128 bit hex string, e.g. `0x1234567890abcdef1234567890abcdef`
    pub async fn place_order(
        &self,
        wallet: Arc<LocalWallet>,
        orders: Vec<OrderRequest>,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Order {
            grouping: Grouping::Na,
            orders,
        };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Place a normal order with tpsl order
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `orders` - The orders to place
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    ///
    /// # Note
    /// * `cloid` in argument `order` is an optional 128 bit hex string, e.g. `0x1234567890abcdef1234567890abcdef`
    pub async fn normal_tpsl(
        &self,
        wallet: Arc<LocalWallet>,
        orders: Vec<OrderRequest>,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Order {
            grouping: Grouping::NormalTpsl,
            orders,
        };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Cancel an order
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `cancels` - The orders to cancel
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    pub async fn cancel_order(
        &self,
        wallet: Arc<LocalWallet>,
        cancels: Vec<CancelRequest>,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Cancel { cancels };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Cancel order(s) by client order id (cloid)
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `cancels` - The client orders to cancel
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    ///
    /// Note: `cloid` in argument `cancel` is a 128 bit hex string, e.g. `0x1234567890abcdef1234567890abcdef`
    pub async fn cancel_order_by_cloid(
        &self,
        wallet: Arc<LocalWallet>,
        cancels: Vec<CancelByCloidRequest>,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::CancelByCloid { cancels };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Modify an order
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `order` - The orders to modify
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    ///
    /// Note: `cloid` in argument `order` is an optional 128 bit hex string, e.g. `0x1234567890abcdef1234567890abcdef`
    pub async fn modify_order(
        &self,
        wallet: Arc<LocalWallet>,
        order: ModifyRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Modify(order);

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Batch modify orders
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `orders` - The orders to modify
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    pub async fn batch_modify_orders(
        &self,
        wallet: Arc<LocalWallet>,
        orders: Vec<ModifyRequest>,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::BatchModify { modifies: orders };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Update cross or isolated leverage on a coin
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `leverage` - The new leverage to set
    /// * `asset` - The asset to set the leverage for
    /// * `is_cross` - true if cross leverage, false if isolated leverage
    pub async fn update_leverage(
        &self,
        wallet: Arc<LocalWallet>,
        leverage: u32,
        asset: u32,
        is_cross: bool,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::UpdateLeverage {
            asset,
            is_cross,
            leverage,
        };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Add or remove margin from isolated position
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `asset` - The asset to set the margin for
    /// * `is_buy` - true if adding margin, false if removing margin
    /// * `ntli` - The new margin to set
    pub async fn update_isolated_margin(
        &self,
        wallet: Arc<LocalWallet>,
        asset: u32,
        is_buy: bool,
        ntli: i64,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::UpdateIsolatedMargin {
            asset,
            is_buy,
            ntli,
        };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Place a TWAP order
    /// # Arguments
    /// * `wallet` - The wallet to sign the order with
    /// * `twap` - The twap order to place
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    pub async fn twap_order(
        &self,
        wallet: Arc<LocalWallet>,
        twap: TwapRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::TwapOrder { twap };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Send usd to another address. This transfer does not touch the EVM bridge. The signature
    /// format is human readable for wallet interfaces.
    ///
    /// # Arguments
    /// * `from` - The wallet to sign the transfer with
    /// * `destination` - The address to send the usd to
    /// * `amount` - The amount of usd to send
    pub async fn usdc_transfer(
        &self,
        from: Arc<LocalWallet>,
        destination: Address,
        amount: String,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let hyperliquid_chain = match self.chain {
            Chain::Arbitrum => HyperliquidChain::Mainnet,
            Chain::ArbitrumTestnet => HyperliquidChain::Testnet,
            _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
        };

        let payload = UsdSend {
            signature_chain_id: 421614.into(),
            hyperliquid_chain,
            amount,
            destination: to_checksum(&destination, None),
            time: nonce,
        };

        let signature = from.sign_typed_data(&payload).await?;

        let action = Action::UsdSend(payload);

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Withdraw from bridge
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the withdrawal with
    /// * `destination` - The address to send the usd to
    /// * `amount` - The amount of usd to send
    pub async fn withdraw_from_bridge(
        &self,
        wallet: Arc<LocalWallet>,
        destination: Address,
        amount: String,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let hyperliquid_chain = match self.chain {
            Chain::Arbitrum => HyperliquidChain::Mainnet,
            Chain::ArbitrumTestnet => HyperliquidChain::Testnet,
            _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
        };

        let payload = Withdraw3 {
            hyperliquid_chain,
            signature_chain_id: 421614.into(),
            amount,
            destination: to_checksum(&destination, None),
            time: nonce,
        };

        let signature = wallet.sign_typed_data(&payload).await?;

        let action = Action::Withdraw3(payload);

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Approve an agent to trade on behalf of the user
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the approval with
    /// * `agent_address` - The address of the agent to approve
    /// * `agent_name` - An optional name for the agent
    pub async fn approve_agent(
        &self,
        wallet: Arc<LocalWallet>,
        agent_address: Address,
        agent_name: Option<String>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let hyperliquid_chain = match self.chain {
            Chain::Arbitrum => HyperliquidChain::Mainnet,
            Chain::ArbitrumTestnet => HyperliquidChain::Testnet,
            _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
        };

        let agent = ApproveAgent {
            hyperliquid_chain,
            signature_chain_id: 421614.into(),
            agent_address,
            nonce,
            agent_name,
        };

        let signature = wallet.sign_typed_data(&agent).await?;

        let action = Action::ApproveAgent(agent);

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Create subaccount for the user
    ///
    /// # Arguments
    /// * `wallet` - The wallet to create the subaccount with
    /// * `name` - The name of the subaccount
    pub async fn create_sub_account(
        &self,
        wallet: Arc<LocalWallet>,
        name: String,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::CreateSubAccount { name };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Rename subaccount
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the rename with
    /// * `name` - The new name of the subaccount
    /// * `sub_account_user` - The address of the subaccount to rename
    pub async fn sub_account_modify(
        &self,
        wallet: Arc<LocalWallet>,
        name: String,
        sub_account_user: Address,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::SubAccountModify {
            name,
            sub_account_user,
        };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Transfer funds between subaccounts
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the transfer with
    /// * `from` - The subaccount to transfer from
    pub async fn sub_account_transfer(
        &self,
        wallet: Arc<LocalWallet>,
        is_deposit: bool,
        sub_account_user: Address,
        usd: u64,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::SubAccountTransfer {
            is_deposit,
            sub_account_user,
            usd,
        };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Set referrer for the user
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the transfer with
    /// * `code` - The referrer code
    pub async fn set_referrer(&self, wallet: Arc<LocalWallet>, code: String) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::SetReferrer { code };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Schedule a time in (UTC ms) to cancel all open orders
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the transaction with
    /// * `time` - Optional time in milliseconds to cancel all open orders
    ///
    /// # Note
    /// * If `time` is `None`, then unsets any cancel time in the future.
    /// `time` must be atleast 5 seconds after the current time
    /// * Once the time is reached, all open orders will be cancelled and trigger count will be incremented.
    /// The max number of triggers is 10 per day. Trigger count resets at 00:00 UTC
    pub async fn schedule_cancel(
        &self,
        wallet: Arc<LocalWallet>,
        time: Option<u64>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let time = time.unwrap_or_else(|| nonce + 5000);

        let action = Action::ScheduleCancel { time };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign_l1_action(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    async fn sign_l1_action(
        &self,
        wallet: Arc<LocalWallet>,
        connection_id: H256,
    ) -> Result<Signature> {
        let source = match self.chain {
            Chain::Arbitrum => "a".to_string(),
            Chain::ArbitrumTestnet => "b".to_string(),
            _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
        };

        let payload = l1::Agent {
            source,
            connection_id,
        };

        Ok(wallet.sign_typed_data(&payload).await?)
    }

    /// get the next nonce to use
    fn nonce(&self) -> Result<u64> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(now.as_millis() as u64)
    }
}
