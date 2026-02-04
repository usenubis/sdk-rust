//! Official Nubis SDK for Rust
//!
//! This SDK provides a type-safe interface to the Nubis API.
//!
//! # Example
//!
//! ```no_run
//! use nubis_sdk::NubisClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = NubisClient::new(
//!         std::env::var("NUBIS_API_KEY")?
//!     );
//!
//!     // List all droplets in a project
//!     let droplets = client
//!         .droplets()
//!         .list("proj_01J5X...")
//!         .await?;
//!
//!     // Create a new droplet
//!     use nubis_sdk::types::CreateDropletRequest;
//!     let droplet = client
//!         .droplets()
//!         .create(CreateDropletRequest {
//!             project_id: "proj_01J5X...".into(),
//!             name: "api-server".into(),
//!             size: "s-1vcpu-1gb".into(),
//!             region: "nyc1".into(),
//!             image: "ubuntu-24.04-x64".into(),
//!             ssh_keys: vec![],
//!             ssh_public_key: None,
//!             ssh_public_key_id: None,
//!             network_id: None,
//!             firewall_id: None,
//!             public_ip: Some(true),
//!             tags: None,
//!             enable_password_auth: None,
//!             admin_password: None,
//!             ssh_allowed_cidrs: vec![],
//!         })
//!         .await?;
//!
//!     println!("Created Droplet: {}", droplet.id);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod resources;
pub mod types;

pub use client::NubisClient;
pub use error::{NubisError, Result};
pub use types::*;

use resources::*;

/// Main Nubis SDK client with resource accessors
impl NubisClient {
    /// Access droplets (VMs) API
    pub fn droplets(&self) -> Droplets {
        Droplets::new(self)
    }

    /// Access projects API
    pub fn projects(&self) -> Projects {
        Projects::new(self)
    }

    /// Access organizations API
    pub fn orgs(&self) -> Orgs {
        Orgs::new(self)
    }

    /// Access regions API
    pub fn regions(&self) -> Regions {
        Regions::new(self)
    }

    /// Access sizes API
    pub fn sizes(&self) -> Sizes {
        Sizes::new(self)
    }

    /// Access images API
    pub fn images(&self) -> Images {
        Images::new(self)
    }
}
