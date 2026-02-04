# nubis-sdk

Official Nubis SDK for Rust.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nubis-sdk = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use nubis_sdk::NubisClient;
use nubis_sdk::types::CreateVmRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NubisClient::new(
        std::env::var("NUBIS_API_KEY")?
    );

    // List all VMs in a project
    let vms = client
        .vms()
        .list("proj_01J5X...")
        .await?;

    // Create a new VM
    let vm = client
        .vms()
        .create(CreateVmRequest {
            project_id: "proj_01J5X...".into(),
            name: "api-server".into(),
            size: "s-1vcpu-1gb".into(),
            region: "nyc1".into(),
            image: "ubuntu-24.04-x64".into(),
            ssh_keys: vec![],
            ssh_public_key: None,
            ssh_public_key_id: None,
            network_id: None,
            firewall_id: None,
            public_ip: Some(true),
            tags: None,
            enable_password_auth: None,
            admin_password: None,
            ssh_allowed_cidrs: vec![],
        })
        .await?;

    println!("Created VM: {}", vm.id);
    Ok(())
}
```

## API Reference

### Resources

- `vms()` - Manage virtual machines
- `projects()` - Manage projects
- `orgs()` - Manage organizations
- `regions()` - List available regions
- `sizes()` - List available VM sizes
- `images()` - List available images/distributions

## License

MIT
