# Agent Instructions

## Mission

Evolve UC Rust as both a proprietary Unified Commerce platform and a high-quality Rust golden path.

## Required reading

Before changing architecture or domain behaviour, read:

1. `CONTEXT.md`
2. relevant files under `docs/adr/`
3. the affected crate public API and tests
4. `governance/github-manifest.json` when changing issues, labels, milestones or Project metadata
5. `docs/governance/release-packaging.md` when changing versions, packaging, publishing or deployment artifacts

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

## GitHub governance

- `governance/github-manifest.json` is the source of truth for repository labels, milestones, managed issues and GitHub Project #4.
- Do not create persistent labels, milestones, project fields or project options manually.
- Every managed issue must be declared in the manifest.
- Undefined GitHub metadata is removed by the governance synchronization workflow in confirmed apply mode.
- Manual metadata changes may be overwritten.
- No feature implementation may resume before the Project Ready gate in issue #19 is approved.

## Release and packaging governance

- Release Please is the only authority that calculates and writes repository release versions.
- Cargo packages, application binaries, container images, Helm charts and GitHub Releases use one coordinated semantic version.
- Do not manually edit release versions except while bootstrapping or through an approved recovery procedure.
- Publishing workflows derive versions from the immutable Git tag created by Release Please.
- Existing tags and immutable artifacts must never be overwritten.
- Partial publication failures are retried from the same tag; unrecoverable inconsistencies require a new patch release.
- Changes to release topology, independent component versions, registries, signing or promotion require an RFC or ADR according to governance policy.

## Validation

Run before proposing changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Architecture changes

Create or update an ADR when changing boundaries, persistence strategy, event delivery, public contracts, security model or deployment topology.
