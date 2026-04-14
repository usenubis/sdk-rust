use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NubisError {
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("http {status}: {message}")]
    Http {
        status: u16,
        message: String,
        body: Value,
    },
}
