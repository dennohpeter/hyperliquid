use ethers::signers::WalletError;
use std::time::SystemTimeError;
use thiserror::Error as ThisError;
use tokio_tungstenite::tungstenite;
use tungstenite::Error as WsError;

use crate::types::websocket::request::Subscription;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Reqwest error: {0:?}")]
    Reqwest(reqwest::Error),
    #[error("Timestamp error: {0:?}")]
    TimestampError(SystemTimeError),
    #[error("Wallet error: {0:?}")]
    WalletError(WalletError),
    #[error("WS error: {0:?}")]
    WsError(WsError),
    #[error("Not connected")]
    NotConnected,
    #[error("JSON error: {0:?}")]
    Json(serde_json::Error),
    #[error("Not subscribed to channel with id {0}")]
    NotSubscribed(u64),
    #[error("Subscription failed: {0:?}")]
    SubscriptionFailed(Subscription),
    #[error("Missing subscription response: {0:?}")]
    MissingSubscriptionResponse(Subscription),
    #[error("Rmp serde error: {0:?}")]
    RmpSerdeError(String),
    #[error("Chain {0} not supported")]
    ChainNotSupported(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<SystemTimeError> for Error {
    fn from(e: SystemTimeError) -> Self {
        Self::TimestampError(e)
    }
}

impl From<WalletError> for Error {
    fn from(e: WalletError) -> Self {
        Self::WalletError(e)
    }
}

impl From<WsError> for Error {
    fn from(e: WsError) -> Self {
        Self::WsError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
