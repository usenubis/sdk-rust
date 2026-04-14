mod error;
pub mod generated;
mod http;

pub use crate::error::NubisError;
pub use crate::http::{NubisClient, NubisClientBuilder};
