use axum::response::IntoResponse;
use thiserror::Error;
use abi::Error as AbiError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
