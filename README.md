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
use nubis_sdk::types::CreateDropletRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NubisClient::new(
        std::env::var("NUBIS_API_KEY")?
    );

    // List all droplets in a project
    let droplets = client
        .droplets()
        .list("proj_01J5X...")
        .await?;

    // Create a new droplet
    let droplet = client
        .droplets()
        .create(CreateDropletRequest {
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

    println!("Created Droplet: {}", droplet.id);
    Ok(())
}
```

## API Reference

### Resources

- `droplets()` - Manage virtual machines (droplets)
- `projects()` - Manage projects
- `orgs()` - Manage organizations
- `regions()` - List available regions
- `sizes()` - List available VM sizes
- `images()` - List available images/distributions

## License

MIT
