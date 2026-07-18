# Session: Self-provisioned test environments

- Date: 2026-07-18
- Governing issue: #27
- Context impact: architecture, testing, CI and delivery governance

## Objective

Define how integration, contract, migration and end-to-end tests obtain databases and other external services locally and in GitHub Actions.

## Decisions crystallized

- Accepted ADR-0010 for self-provisioned ephemeral test environments.
- Adopted Testcontainers for test-scoped dependencies and adapter contract suites.
- Adopted Docker Compose for multi-service and end-to-end environments reproducible by developers.
- Limited GitHub Actions service containers to simple job-wide dependencies.
- Required pinned versions, readiness probes, isolated state, automatic teardown and failure diagnostics.
- Added the self-provisioned integration pipeline to the Project Ready gate.

## Repository changes

- `.context/decisions/ADR-0010-self-provisioned-ephemeral-test-environments.md`
- `AGENTS.md`
- `governance/github-manifest.json`
- GitHub issue #27
- GitHub issue #19 readiness checklist

## Open implementation work

- Select concrete Rust Testcontainers crates and versions.
- Define Docker Compose profiles and directory layout.
- Implement PostgreSQL, Redis and S3-compatible test environments as required by real adapters.
- Add log and diagnostic artifact collection to GitHub Actions.
- Prove clean-runner and local reproducibility before Project Ready approval.
