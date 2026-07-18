# AGENTS.md

## Purpose

This repository contains both the UC Rust product and the engineering golden path used to build it.

Agents must preserve domain boundaries, repository context and project governance. They must not optimize locally at the expense of the target architecture or accepted decisions.

## Required context

Before changing the repository, read in this order:

1. `PROJECT_CHARTER.md`, when present;
2. `CONTEXT.md`;
3. `ARCHITECTURE.md` and relevant files under `docs/architecture/`;
4. accepted ADRs and RFCs relevant to the work;
5. the governing GitHub issue and its dependencies;
6. this file;
7. `governance/github-manifest.json` for backlog and metadata changes.

When sources conflict, accepted ADRs and RFCs override descriptive documentation. The Project Ready gate controls whether implementation may resume.

## Implementation gate

Feature implementation is blocked until issue #19 records an approved Project Ready review.

Until that gate is approved, permitted work is limited to project definition, architecture, agentic operating model, repository memory, governance, security, quality gates and roadmap planning.

## GitHub governance

Repository labels, milestones, managed issue metadata and Project #4 are declarative.

- `governance/github-manifest.json` is the source of truth.
- Do not create permanent labels, milestones or Project fields only through the UI.
- Every managed issue must be represented in the manifest.
- Release is represented by milestone, not by a release label.
- Workflow state and size are Project fields, not repository labels.
- Undefined metadata is removed by the governance apply workflow.
- Destructive synchronization must use `.github/workflows/sync-governance.yml` apply mode and its explicit confirmation gate.

See `docs/governance/github-metadata.md`.

## Engineering rules

- Start from a GitHub issue with clear acceptance criteria.
- Keep changes within the issue scope.
- Keep `uc-domain` free from HTTP, database, messaging and framework dependencies.
- Express business invariants through types and domain methods.
- Do not use floating point for money.
- Do not introduce a dependency without documenting ownership, operational impact, license and supply-chain risk.
- Prefer a small vertical slice over broad scaffolding once implementation is unblocked.
- Every bug fix requires a regression test.
- Public contracts and domain events must be versioned before external adoption.
- Avoid `unwrap`, `expect` and panics in production paths.
- Unsafe Rust is forbidden unless a dedicated ADR explicitly changes the policy.
- Do not silently change public contracts, domain terminology or bounded-context ownership.
- Do not commit credentials, secrets, tokens or production data.

## Agent handoff

Every handoff must state:

- governing issue;
- files changed;
- evidence and checks performed;
- decisions proposed or accepted;
- unresolved risks and assumptions;
- exact next action.

Partial work must remain explicit. Temporary notes are not accepted architecture until consolidated into the appropriate durable document.

## Validation

For Rust code:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

For governance changes:

```bash
python3 scripts/sync_github_governance.py --mode validate
```

A pull request must link its issue and include validation evidence.

## Architecture changes

Create or update an ADR or RFC when changing boundaries, persistence strategy, event delivery, public contracts, security model or deployment topology.
