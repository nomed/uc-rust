# ADR-0006 — Forward-only database migrations with expand/migrate/contract

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #23
- Decision owner: Human project owner

## Context

UC Rust will evolve a production database while application versions may overlap during deployment. Editing old patches, depending on manual fixes or applying destructive schema changes in one step creates drift, failed upgrades and incompatibility between running binaries.

## Decision

Database schema evolution will use ordered, immutable, forward-only migrations stored in source control.

Once a migration has been applied to any shared environment, it must never be edited, reordered or reused. Corrections are introduced by a new migration.

Breaking changes use the expand/migrate/contract pattern:

1. **Expand**: add backward-compatible schema structures.
2. **Migrate**: backfill or move data while old and new application versions remain compatible.
3. **Contract**: remove obsolete structures only after compatibility and usage checks confirm they are no longer required.

Production rollback means rolling back the application to a schema-compatible version or issuing a forward corrective migration. Down migrations are not the primary production recovery mechanism.

## Required properties

- deterministic, monotonic migration identifiers;
- committed migration history and checksums;
- automatic drift detection;
- transactional execution when supported by PostgreSQL;
- explicit handling of non-transactional DDL;
- clean-install testing from an empty database;
- supported upgrade-path testing from released versions;
- rolling-deployment compatibility testing;
- long-running data backfills separated from short schema migrations;
- explicit recovery procedures for partial failure;
- one declared owner for migration execution.

## Runtime policy

Application processes must not race to apply migrations implicitly in production. Migration execution will be an explicit release/deployment step or dedicated job with controlled permissions and observability.

The application may verify schema compatibility at startup and fail clearly when the database is outside its supported range.

## Structural rules

- SQL migration files contain schema changes, not hidden application business logic.
- Data corrections that encode domain meaning must use reviewed application/backfill code where appropriate.
- Destructive DDL requires a compatibility plan and evidence that old binaries and data consumers no longer depend on the removed structure.
- Tenant-specific or country-specific differences should be represented in data/configuration or explicit bounded modules, not uncontrolled schema forks.

## Tooling

The concrete migration runner will be selected before PostgreSQL persistence implementation begins. SQLx migrations are a candidate, but tooling must satisfy checksum, history, offline validation and operational control requirements before acceptance.

## Consequences

### Positive

- Environments can be reconstructed and compared.
- Rolling releases remain safe.
- Schema history is auditable.
- Failed changes have a clear forward recovery model.

### Negative

- Breaking changes require multiple releases or deployment phases.
- Temporary duplicated columns/tables may exist during transitions.
- Backfill operations require explicit planning and monitoring.

## Alternatives rejected

- Editing already-applied migration files: causes environment drift and destroys auditability.
- Automatic destructive migrations generated from models: insufficient operational control.
- Down migrations as the main rollback strategy: data loss and irreversible transformations make this unreliable in production.
- Manual DBA patches outside source control: cannot be reproduced or verified consistently.
