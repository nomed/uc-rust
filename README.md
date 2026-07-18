# UC Rust

A proprietary Unified Commerce implementation in Rust, built together with its own Rust golden path.

## Purpose

This repository has two inseparable responsibilities:

1. define and maintain the Rust golden path used by the platform;
2. implement the Unified Commerce kernel using that golden path.

UC-BoK is the semantic and architectural reference for capabilities, processes, business objects, lifecycles, events, application responsibilities, integration patterns and assessments.

## Initial architecture

The project starts as a modular monolith organised by business capability. Physical service extraction is allowed only when justified by independent scaling, isolation, ownership or release requirements.

Initial vertical slice:

```text
Create Basket
→ Add Product
→ Calculate Basket
→ Start Checkout
→ Confirm Order
→ Publish OrderConfirmed
```

## Workspace

```text
crates/uc-domain       pure domain model and invariants
crates/uc-application  use cases and ports
crates/uc-api          HTTP API and transport adapters
apps/uc-server         executable composition root
docs/adr               architecture decisions
```

## Local quality gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Status

Early foundation. The repository is intentionally small and executable before adding infrastructure, persistence and messaging.
