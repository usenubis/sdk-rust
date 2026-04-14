# Nubis Rust SDK

Official Rust SDK for Nubis.

Designed for backend services and platform automation that need:

- broad API coverage with generated endpoint methods
- explicit request control (method, path params, query, body)
- consistent error types for production monitoring and retries

## Install

From crates.io:

```toml
[dependencies]
nubis-sdk = "1.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

For local development:

```toml
[dependencies]
nubis-sdk = { path = "../nubis-core/sdk-rust" }
```

## Quick Start

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

## How Endpoint Methods Are Named

Endpoint methods are generated from backend routes and follow:

`<http_method>_<normalized_path>`

Examples:

- `GET /api/v1/orgs` -> `get_api_v1_orgs(...)`
- `POST /api/v1/orgs/:org_id/projects` -> `post_api_v1_orgs_by_org_id_projects(...)`

Path params become function arguments in order. Query/body are passed explicitly.

## Calling Patterns

Read-style routes:

```rust
let data = client
    .get_api_v1_projects_by_project_id("proj_123", None)
    .await?;
```

Write-style routes:

```rust
let payload = serde_json::json!({ "name": "new-name" });
let data = client
    .post_api_v1_orgs(Some(&payload), None)
    .await?;
```

With query params:

```rust
let query = [("range_minutes", "60")];
let metrics = client
    .get_api_v1_orgs_by_org_id_metrics_compute("org_123", Some(&query))
    .await?;
```

## Error Handling

All SDK errors use `NubisError`:

- `NubisError::Transport(reqwest::Error)`
- `NubisError::Serialization(serde_json::Error)`
- `NubisError::Http { status, message, body }`

```rust
match client.get_api_v1_orgs(None).await {
    Ok(data) => println!("{data}"),
    Err(nubis_sdk::NubisError::Http { status, message, .. }) => {
        eprintln!("request failed: {status} - {message}");
    }
    Err(err) => eprintln!("sdk error: {err}"),
}
```

## What Is Generated

- Route methods are generated from:
  - `services/api-gateway/src/main.rs`
- Generated output file:
  - `src/generated/endpoints.rs`

This ensures coverage keeps pace with backend route additions.

## Regeneration Workflow

Run this after route changes:

```bash
python scripts/generate_sdk_rust.py
cargo fmt
cargo check
```

## Publishing to crates.io

1. Update `version` in `Cargo.toml`
2. Validate package:
   `cargo publish --dry-run`
3. Publish:
   `cargo publish`
4. Verify:
   [crates.io/nubis-sdk](https://crates.io/crates/nubis-sdk)

## License

MIT. See [LICENSE.md](./LICENSE.md).

## Related SDK

Need JavaScript or TypeScript? See:

- `../sdk-js/README.md`
