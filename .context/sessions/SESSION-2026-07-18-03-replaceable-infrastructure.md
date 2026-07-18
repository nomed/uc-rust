# Session 2026-07-18-03: Replaceable infrastructure adapters

- Date: 2026-07-18
- Governing issue: #24
- Status: Completed

## Objective

Crystallize the requirement that persistence, cache and object/file storage providers remain replaceable without changing domain or application contracts.

## Inputs

- User requirement to support switching PostgreSQL, SQLite and future databases.
- User requirement to support interchangeable cache providers.
- User requirement to support S3-compatible, filesystem and future storage providers.
- ADR-0005 single application core.
- ADR-0006 forward-only database migration policy.

## Actions

- Created issue #24.
- Accepted ADR-0007.
- Updated `AGENTS.md` with replaceable infrastructure and capability-contract rules.
- Added issue #24 to `governance/github-manifest.json`.

## Decisions crystallized

- Application and domain contracts are provider-neutral and capability-oriented.
- Provider SDK and transport types cannot leak inward.
- Provider switching occurs at the composition root.
- Shared behavioral contract tests apply to every adapter.
- Backend neutrality must not silently weaken semantics.
- Database migrations remain provider-specific infrastructure assets.

## Evidence

- `.context/decisions/ADR-0007-replaceable-infrastructure-adapters.md`
- `AGENTS.md`
- `governance/github-manifest.json`
- GitHub issue #24

## Open questions

- Exact crate layout for ports and provider adapters.
- Initial cache provider and production consistency requirements.
- Initial object storage provider and streaming requirements.
- PostgreSQL and SQLite compatibility scope for the first production release.
