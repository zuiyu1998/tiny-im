use abi::serde_json::Error as SerdeJsonError;
use abi::Error as AbiError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("serde json error: {0}")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("nacos error: {0}")]
    NacosError(String),
    #[error("join error: {0}")]
    JoinError(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
