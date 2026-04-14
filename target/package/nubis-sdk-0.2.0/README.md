# Nubis Rust SDK

Official Rust client for the Nubis API.

## Features

- Typed HTTP client with bearer-token auth support
- Full route coverage generated from `services/api-gateway/src/main.rs`
- Generic JSON request/response support for rapid integration

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
nubis-sdk = { path = "../nubis-core/sdk-rust" }
```

## Usage

```rust
use nubis_sdk::NubisClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), nubis_sdk::NubisError> {
    let client = NubisClient::builder()
        .base_url("https://nubis-core.onrender.com")
        .api_key("your_api_key")
        .build()?;

    let orgs = client.get_api_v1_orgs(None).await?;
    println!("orgs: {orgs}");

    let payload = json!({ "name": "acme", "slug": "acme" });
    let created = client.post_api_v1_orgs(Some(&payload), None).await?;
    println!("created: {created}");

    Ok(())
}
```

## Development

Regenerate endpoints after backend route changes:

```bash
python scripts/generate_sdk_rust.py
cargo check
```

