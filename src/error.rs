<<<<<<< HEAD
use thiserror::Error;

/// Errors that can occur when using the Nubis SDK
#[derive(Debug, Error)]
pub enum NubisError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, NubisError>;
=======
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
>>>>>>> 0af0069 (Initial import)
