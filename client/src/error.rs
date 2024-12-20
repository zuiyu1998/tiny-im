use abi::Error as AbiError;
use thiserror::Error;
use abi::reqwest::Error as ReqwestError;
use abi::serde_json::Error as SerdeJsonError;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("ServerError")]
    ServerError,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] ErrorKind),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),
    #[error("serde json error: {0}")]
    SerdeJsonError(#[from] SerdeJsonError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
