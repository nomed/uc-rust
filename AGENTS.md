# Agent Instructions

## Mission

Evolve UC Rust as both a proprietary Unified Commerce platform and a high-quality Rust golden path.

## Required reading

Before any meaningful work, read in this order:

1. `.context/manifest.yaml`
2. `.context/README.md`
3. this `AGENTS.md` and any nearer nested `AGENTS.md`
4. accepted records under `.context/decisions/`
5. accepted records under `.context/rfcs/` relevant to the task
6. the governing GitHub issue and latest applicable handoff
7. `CONTEXT.md`, project charter and target architecture documents relevant to the task
8. the affected crate public API and tests

Also read:

- `governance/github-manifest.json` when changing issues, labels, milestones or Project metadata;
- `docs/governance/release-packaging.md` when changing versions, packaging, publishing or deployment artifacts.

When sources conflict, follow the precedence declared in `.context/manifest.yaml`. Do not guess or silently reconcile material conflicts.

## Context records

- `.context/` is the durable operating memory for humans and agents.
- Sessions and handoffs preserve continuity and evidence but are not architecture authority.
- Material decisions must be recorded as numbered ADRs or RFCs.
- Agents may draft ADRs and RFCs; human approval is required to mark material decisions accepted.
- Accepted records are immutable. Replace them only through a new record that marks the old one superseded.
- At the end of meaningful work, record a session and create a handoff when continuation is required.
- Never store secrets, private chain-of-thought or unredacted sensitive data in context records.

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

Run before proposing code changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Validate context and governance manifests when they are affected.

## Architecture changes

Create or update a decision record when changing boundaries, persistence strategy, event delivery, public contracts, security model, deployment topology, release model or agentic operating model. Substantial or high-cost changes require an RFC before implementation.