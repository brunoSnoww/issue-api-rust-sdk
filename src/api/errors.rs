use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(#[from] ReqwestError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeJsonError),
}
