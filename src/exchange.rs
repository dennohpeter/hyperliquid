use std::{sync::Arc, time::SystemTime};

use ethers::{
    abi::AbiEncode,
    signers::{LocalWallet, Signer},
    types::{Address, Signature, H256},
    utils::{keccak256, to_checksum},
};

use crate::{
    client::Client,
    error::Result,
    types::{
        agent::{l1, mainnet, testnet},
        exchange::{
            request::{
                Action, Agent, CancelByCloidRequest, CancelRequest, Chain, Grouping, OrderRequest,
                Request, TransferRequest,
            },
            response::Response,
        },
        usd_transfer, API,
    },
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
    /// * `order` - The order to place
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    ///  e.g. `0x0000000000000000000000000000000000000000`
    ///
    ///  Note: `cloid` in argument `order` is an optional 128 bit hex string, e.g. `0x1234567890abcdef1234567890abcdef`
    pub async fn place_order(
        &self,
        wallet: Arc<LocalWallet>,
        order: OrderRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Order {
            grouping: Grouping::Na,
            orders: vec![order],
        };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign(wallet, connection_id).await?;

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
    /// * `cancel` - The order to cancel
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    pub async fn cancel_order(
        &self,
        wallet: Arc<LocalWallet>,
        cancel: CancelRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Cancel {
            cancels: vec![cancel],
        };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign(wallet, connection_id).await?;

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
    /// * `cancel` - The client order to cancel
    /// * `vault_address` - If trading on behalf of a vault, its onchain address in 42-character hexadecimal format
    /// e.g. `0x0000000000000000000000000000000000000000`
    ///
    /// Note: `cloid` in argument `cancel` is a 128 bit hex string, e.g. `0x1234567890abcdef1234567890abcdef`
    pub async fn cancel_order_by_cloid(
        &self,
        wallet: Arc<LocalWallet>,
        cancel: CancelByCloidRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::CancelByCloid {
            cancels: vec![cancel],
        };

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// L1 USDC transfer
    pub async fn usdc_transfer(
        &self,
        from: Arc<LocalWallet>,
        destination: Address,
        amount: String,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let signature = {
            let destination = to_checksum(&destination, None);

            match self.chain {
                Chain::Arbitrum => {
                    from.sign_typed_data(&usd_transfer::mainnet::UsdTransferSignPayload {
                        destination,
                        amount: amount.clone(),
                        time: nonce as u64,
                    })
                    .await?
                }
                Chain::ArbitrumGoerli => {
                    from.sign_typed_data(&usd_transfer::testnet::UsdTransferSignPayload {
                        destination,
                        amount: amount.clone(),
                        time: nonce as u64,
                    })
                    .await?
                }
                Chain::Dev => todo!("Dev chain not supported"),
            }
        };

        let payload = TransferRequest {
            amount,
            destination: to_checksum(&destination, None),
            time: nonce,
        };

        let action = Action::UsdTransfer {
            chain: self.chain,
            payload,
        };

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Initiate a withdrawal request
    pub async fn withdraw(&self, from: Arc<LocalWallet>, usd: String) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::Withdraw { usd, nonce };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign(from, connection_id).await?;

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

        let signature = self.sign(wallet, connection_id).await?;

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
    /// * `margin` - The new margin to set
    /// * `asset` - The asset to set the margin for
    pub async fn update_isolated_margin(
        &self,
        wallet: Arc<LocalWallet>,
        margin: i64,
        asset: u32,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::UpdateIsolatedMargin {
            asset,
            is_buy: true,
            ntli: margin,
        };

        let vault_address = None;

        let connection_id = action.connection_id(vault_address, nonce)?;

        let signature = self.sign(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Approve an agent to trade on behalf of the user
    pub async fn approve_agent(
        &self,
        wallet: Arc<LocalWallet>,
        agent_address: Address,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let connection_id = keccak256(agent_address.encode()).into();

        let action = Action::ApproveAgent {
            chain: match self.chain {
                Chain::Arbitrum => Chain::Arbitrum,
                Chain::Dev | Chain::ArbitrumGoerli => Chain::ArbitrumGoerli,
            },
            agent: Agent {
                source: "https://hyperliquid.xyz".to_string(),
                connection_id,
            },
            agent_address,
        };

        let signature = self.sign(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Create a signature for the given connection id
    async fn sign(&self, wallet: Arc<LocalWallet>, connection_id: H256) -> Result<Signature> {
        let (chain, source) = match self.chain {
            Chain::Arbitrum => (Chain::Dev, "a".to_string()),
            Chain::Dev | Chain::ArbitrumGoerli => (Chain::Dev, "b".to_string()),
        };

        Ok(match chain {
            Chain::Arbitrum => {
                let payload = mainnet::Agent {
                    source,
                    connection_id,
                };
                wallet.sign_typed_data(&payload).await?
            }
            Chain::ArbitrumGoerli => {
                let payload = testnet::Agent {
                    source,
                    connection_id,
                };
                wallet.sign_typed_data(&payload).await?
            }
            Chain::Dev => {
                let payload = l1::Agent {
                    source,
                    connection_id,
                };

                wallet.sign_typed_data(&payload).await?
            }
        })
    }

    /// get the next nonce to use
    fn nonce(&self) -> Result<u128> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(now.as_millis())
    }
}
