# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

## [1.2.0] - 2026-04-14

### Added

- Route-driven endpoint code generation from backend gateway routes (`services/api-gateway/src/main.rs`).
- Generated endpoint client coverage for current API surface (method-specific functions with path parameter mapping).
- Core Rust SDK primitives:
  - `NubisClient` and `NubisClientBuilder`
  - JSON request helpers (`request_value`, `request`)
  - auth header handling for bearer API keys
  - percent-encoded path parameter rendering
- Unified SDK error type (`NubisError`) with transport, serialization, and HTTP variants.
- Comprehensive README with installation, usage, calling patterns, and publishing workflow.

### Changed

- Updated package metadata and release readiness for crates.io publication.
- Regeneration workflow documented and aligned to backend route evolution.

## [1.0.2] - 2026-04-13

### Added

- Previous crates.io release baseline.
