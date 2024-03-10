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
                Action, Agent, CancelByCloidRequest, CancelRequest, Grouping, ModifyRequest,
                OrderRequest, Request, TransferRequest, WithdrawalRequest,
            },
            response::Response,
        },
        usd_transfer, Chain, API,
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

        let signature = self.sign(wallet, connection_id).await?;

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

        let signature = self.sign(wallet, connection_id).await?;

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

        let signature = self.sign(wallet, connection_id).await?;

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

        let signature = {
            let destination = to_checksum(&destination, None);
            let time = nonce;
            let amount = amount.clone();

            match self.chain {
                Chain::Arbitrum => {
                    from.sign_typed_data(&usd_transfer::mainnet::UsdTransferSignPayload {
                        destination,
                        amount,
                        time,
                    })
                    .await?
                }
                Chain::ArbitrumTestnet => {
                    from.sign_typed_data(&usd_transfer::testnet::UsdTransferSignPayload {
                        destination,
                        amount,
                        time,
                    })
                    .await?
                }
                _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
            }
        };

        let payload = TransferRequest {
            amount,
            destination: to_checksum(&destination, None),
            time: nonce,
        };
        let action = Action::UsdTransfer {
            chain: match self.chain {
                Chain::Arbitrum => Chain::Arbitrum,
                _ => Chain::ArbitrumTestnet,
            },
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

    /// Withdraw from bridge
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the withdrawal with
    /// * `destination` - The address to send the usd to
    /// * `usd` - The amount of usd to send
    pub async fn withdraw_from_bridge(
        &self,
        wallet: Arc<LocalWallet>,
        destination: Address,
        usd: String,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let signature = {
            let destination = to_checksum(&destination, None);
            let time = nonce;
            let usd = usd.clone();

            match self.chain {
                Chain::Arbitrum => {
                    wallet
                        .sign_typed_data(&usd_transfer::mainnet::WithdrawFromBridge2SignPayload {
                            destination,
                            usd,
                            time,
                        })
                        .await?
                }
                Chain::ArbitrumTestnet => {
                    wallet
                        .sign_typed_data(&usd_transfer::testnet::WithdrawFromBridge2SignPayload {
                            destination,
                            usd,
                            time,
                        })
                        .await?
                }
                _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
            }
        };

        let payload = WithdrawalRequest {
            usd,
            destination: to_checksum(&destination, None),
            time: nonce,
        };
        let action = Action::Withdraw2 {
            chain: match self.chain {
                Chain::Arbitrum => Chain::Arbitrum,
                _ => Chain::ArbitrumTestnet,
            },
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

    /// Approve an agent to trade on behalf of the user
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the approval with
    /// * `agent_address` - The address of the agent to approve
    /// * `extra_agent_name` - An optional name for the agent
    pub async fn approve_agent(
        &self,
        wallet: Arc<LocalWallet>,
        agent_address: Address,
        extra_agent_name: Option<String>,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let connection_id = keccak256(if let Some(ref name) = extra_agent_name {
            (agent_address, name.to_string()).encode()
        } else {
            agent_address.encode()
        })
        .into();

        let action = Action::Connect {
            chain: match self.chain {
                Chain::Arbitrum => Chain::Arbitrum,
                Chain::ArbitrumTestnet => Chain::ArbitrumTestnet,
                _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
            },
            agent: Agent {
                source: "https://hyperliquid.xyz".to_string(),
                connection_id,
            },
            agent_address,
            extra_agent_name,
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

    /// Initiate a withdrawal request
    ///
    /// # Arguments
    /// * `from` - The wallet to sign the withdrawal with
    /// * `usd` - The amount of usd to send
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

    /// Create subaccount for the user
    ///
    /// # Arguments
    /// * `wallet` - The wallet to create the subaccount with
    /// * `name` - The name of the subaccount
    pub async fn create_subaccount(
        &self,
        wallet: Arc<LocalWallet>,
        name: String,
    ) -> Result<Response> {
        let nonce = self.nonce()?;

        let action = Action::CreateSubAccount { name };

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

    /// Transfer funds between subaccounts
    ///
    /// # Arguments
    /// * `wallet` - The wallet to sign the transfer with
    /// * `from` - The subaccount to transfer from
    pub async fn subaccount_transfer() -> Result<Response> {
        todo!("Implement subaccount transfer")
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

        let signature = self.sign(wallet, connection_id).await?;

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

        let signature = self.sign(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Create a signature for the given connection id
    async fn sign(&self, wallet: Arc<LocalWallet>, connection_id: H256) -> Result<Signature> {
        let (chain, source) = match self.chain {
            Chain::Arbitrum => (Chain::Dev, "a".to_string()),
            Chain::Dev | Chain::ArbitrumGoerli | Chain::ArbitrumTestnet => {
                (Chain::Dev, "b".to_string())
            }
            _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
        };

        Ok(match chain {
            Chain::Arbitrum => {
                let payload = mainnet::Agent {
                    source,
                    connection_id,
                };
                wallet.sign_typed_data(&payload).await?
            }
            Chain::ArbitrumTestnet => {
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

            _ => return Err(Error::ChainNotSupported(self.chain.to_string())),
        })
    }

    /// get the next nonce to use
    fn nonce(&self) -> Result<u64> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(now.as_millis() as u64)
    }
}
