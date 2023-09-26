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
        exchange::request::{
            Action, Agent, CancelRequest, Chain, Grouping, OrderRequest, Request, TransferRequest,
        },
        exchange::response::Response,
        usd_transfer, API,
    },
    utils::float_to_int_for_hashing,
};

/// Endpoint to interact with and trade on the Hyperliquid chain.
pub struct Exchange {
    pub client: Client,
    pub chain: Chain,
}

impl Exchange {
    /// Place an order
    pub async fn place_order(
        &self,
        wallet: Arc<LocalWallet>,
        order: OrderRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.timestamp()?;

        let action = Action::Order {
            grouping: Grouping::Na,
            orders: vec![order],
        };

        let connection_id =
            self.connection_id(&action, vault_address.unwrap_or_default(), nonce)?;

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
    pub async fn cancel_order(
        &self,
        wallet: Arc<LocalWallet>,
        cancel: CancelRequest,
        vault_address: Option<Address>,
    ) -> Result<Response> {
        let nonce = self.timestamp()?;

        let action = Action::Cancel {
            cancels: vec![cancel],
        };

        let connection_id =
            self.connection_id(&action, vault_address.unwrap_or_default(), nonce)?;

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
        let nonce = self.timestamp()?;

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
        let nonce = self.timestamp()?;

        let action = Action::Withdraw { usd, nonce };

        let connection_id = self.connection_id(&action, Address::zero(), nonce)?;

        let signature = self.sign(from, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Update leverage for a given asset
    pub async fn update_leverage(
        &self,
        wallet: Arc<LocalWallet>,
        leverage: u32,
        asset: u32,
        is_cross: bool,
    ) -> Result<Response> {
        let nonce = self.timestamp()?;

        let action = Action::UpdateLeverage {
            asset,
            is_cross,
            leverage,
        };

        let connection_id = self.connection_id(&action, Address::zero(), nonce)?;

        let signature = self.sign(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Update isolated margin for a given asset
    pub async fn update_isolated_margin(
        &self,
        wallet: Arc<LocalWallet>,
        margin: i64,
        asset: u32,
    ) -> Result<Response> {
        let nonce = self.timestamp()?;

        let action = Action::UpdateIsolatedMargin {
            asset,
            is_buy: true,
            ntli: margin,
        };

        let connection_id = self.connection_id(&action, Address::zero(), nonce)?;

        let signature = self.sign(wallet, connection_id).await?;

        let request = Request {
            action,
            nonce,
            signature,
            vault_address: None,
        };

        self.client.post(&API::Exchange, &request).await
    }

    /// Approve an agent to trade on behalf of the user
    pub async fn approve_agent(
        &self,
        wallet: Arc<LocalWallet>,
        agent_address: Address,
    ) -> Result<Response> {
        let nonce = self.timestamp()?;

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

    /// create connection id for agent
    fn connection_id(&self, action: &Action, vault_address: Address, nonce: u128) -> Result<H256> {
        let encoded = match action {
            Action::Order { grouping, orders } => {
                let hashable_tuples = orders
                    .iter()
                    .map(|order| {
                        let order_type = order.get_type();

                        (
                            order.asset,
                            order.is_buy,
                            float_to_int_for_hashing(
                                order.limit_px.parse().expect("Failed to parse limit_px"),
                            ),
                            float_to_int_for_hashing(order.sz.parse().expect("Failed to parse sz")),
                            order.reduce_only,
                            order_type.0,
                            order_type.1,
                        )
                    })
                    .collect::<Vec<_>>();

                (hashable_tuples, grouping.to_i32(), vault_address, nonce).encode()
            }
            Action::Cancel { cancels } => {
                let hashable_tuples = cancels.iter().map(|c| (c.asset, c.oid)).collect::<Vec<_>>();

                (hashable_tuples, vault_address, nonce).encode()
            }
            Action::UsdTransfer {
                chain: _,
                payload: _,
            } => {
                todo!()
            }
            Action::Withdraw { usd: _, nonce: _ } => todo!(),
            Action::UpdateLeverage {
                asset,
                is_cross,
                leverage,
            } => (*asset, *is_cross, *leverage, vault_address, nonce).encode(),
            Action::UpdateIsolatedMargin {
                asset,
                is_buy: _,
                ntli,
            } => (*asset, true, *ntli, vault_address, nonce).encode(),
            Action::ApproveAgent {
                chain: _,
                agent: _,
                agent_address,
            } => agent_address.encode(),
        };

        Ok(keccak256(encoded).into())
    }

    /// Create a signature for the given connection id
    async fn sign(&self, wallet: Arc<LocalWallet>, connection_id: H256) -> Result<Signature> {
        Ok(match self.chain {
            Chain::Arbitrum => {
                let payload = mainnet::Agent {
                    source: "b".to_string(),
                    connection_id,
                };
                wallet.sign_typed_data(&payload).await?
            }
            Chain::ArbitrumGoerli => {
                let payload = testnet::Agent {
                    source: "b".to_string(),
                    connection_id,
                };
                wallet.sign_typed_data(&payload).await?
            }
            Chain::Dev => {
                let payload = l1::Agent {
                    source: "b".to_string(),
                    connection_id,
                };

                wallet.sign_typed_data(&payload).await?
            }
        })
    }

    /// current timestamp in milliseconds
    fn timestamp(&self) -> Result<u128> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(now.as_millis())
    }
}
