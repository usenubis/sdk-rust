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
//!     // List all VMs in a project
//!     let vms = client
//!         .vms()
//!         .list("proj_01J5X...")
//!         .await?;
//!
//!     // Create a new VM
//!     use nubis_sdk::types::CreateVmRequest;
//!     let vm = client
//!         .vms()
//!         .create(CreateVmRequest {
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
//!     println!("Created VM: {}", vm.id);
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
    /// Access VMs API
    pub fn vms(&self) -> Vms<'_> {
        Vms::new(self)
    }

    /// Access projects API
    pub fn projects(&self) -> Projects<'_> {
        Projects::new(self)
    }

    /// Access organizations API
    pub fn orgs(&self) -> Orgs<'_> {
        Orgs::new(self)
    }

    /// Access regions API
    pub fn regions(&self) -> Regions<'_> {
        Regions::new(self)
    }

    /// Access sizes API
    pub fn sizes(&self) -> Sizes<'_> {
        Sizes::new(self)
    }

    /// Access images API
    pub fn images(&self) -> Images<'_> {
        Images::new(self)
    }
}
