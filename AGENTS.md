# Agent Instructions

## Mission

Evolve UC Rust as both a proprietary Unified Commerce platform and a high-quality Rust golden path.

## Required reading

Before changing architecture or domain behaviour, read:

1. `CONTEXT.md`
2. relevant files under `docs/adr/`
3. the affected crate public API and tests

## Engineering rules

- Keep `uc-domain` free from HTTP, database, messaging and framework dependencies.
- Express business invariants through types and domain methods.
- Do not use floating point for money.
- Do not introduce a dependency without documenting its role and maintenance implications.
- Prefer a small vertical slice over broad scaffolding.
- Every bug fix requires a regression test.
- Public contracts and domain events must be versioned before external adoption.
- Avoid `unwrap`, `expect` and panics in production paths.
- Unsafe Rust is forbidden unless a dedicated ADR explicitly changes the policy.

## Validation

Run before proposing changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Architecture changes

Create or update an ADR when changing boundaries, persistence strategy, event delivery, public contracts, security model or deployment topology.
