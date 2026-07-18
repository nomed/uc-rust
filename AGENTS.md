# Agent Instructions

## Mission

Evolve UC Rust as a proprietary Unified Commerce platform and an exemplary Rust golden path without allowing code, context, security, reliability or quality controls to diverge.

## Required reading

Before meaningful work, read in this order:

1. `.context/manifest.yaml`
2. `.context/README.md`
3. this `AGENTS.md` and any nearer nested `AGENTS.md`
4. accepted records under `.context/decisions/`
5. accepted records under `.context/rfcs/` relevant to the task
6. `.context/quality-attributes/system-quality-model.md`
7. the governing GitHub issue and latest applicable handoff
8. project charter, target architecture, affected public APIs and tests

Also read `governance/github-manifest.json` for GitHub metadata and `docs/governance/release-packaging.md` for release work. Follow `.context/manifest.yaml` precedence and never silently reconcile material conflicts.

## Context is part of completion

- `.context/` is durable operating memory, not optional documentation.
- Every meaningful change declares context impact and creates a session record.
- Incomplete or delegated work requires a handoff.
- Material decisions must be promoted into numbered ADRs or RFCs.
- Accepted records are immutable; supersede them with a new record.
- A concrete `no context impact` justification is required when applicable.
- Work is incomplete while code and authoritative context disagree.
- Never store secrets, private chain-of-thought or unredacted sensitive data.

## System quality completeness

- Every P0 quality requirement must appear in `.context/quality-attributes/system-quality-model.md`.
- Each row requires a measurable invariant or budget, owner, governing record/issue, automated enforcement and evidence.
- Exceptions require reason, compensating control, owner and expiry.
- Expired or undocumented exceptions fail the Project Ready gate.

## Single application core

- Use hexagonal / ports-and-adapters architecture with use-case-oriented application operations.
- Every business operation has one canonical implementation.
- Domain types own invariants and state transitions.
- Application operations own orchestration, transactions, idempotency and process flow.
- REST, gRPC, consumers, workers, jobs, CLI and batch processes are inbound adapters only.
- Inbound adapters may authenticate, validate transport syntax, map DTOs, establish technical context, invoke an operation and map the result.
- Adapters must not contain business workflows, pricing, eligibility, lifecycle decisions or duplicated branching.
- Infrastructure remains behind outbound ports; generic utilities must not hide business behavior.

## Replaceable infrastructure adapters

- Domain and application crates depend on capability-oriented ports, never provider SDKs.
- PostgreSQL, SQLite and future databases are persistence adapters.
- Redis and in-memory products are cache adapters.
- S3-compatible stores and filesystems are storage adapters.
- SpiceDB is the initial authorization adapter, not an application-core dependency.
- Provider-specific types, connections, paths, buckets, tokens and errors must not leak inward.
- Provider selection occurs in the composition root.
- Ports state required guarantees: transactions, consistency, ordering, TTL, invalidation, streaming, range reads and idempotency.
- Unsupported guarantees fail explicitly; silent semantic degradation is forbidden.
- Every adapter passes the same reusable contract suite.
- Avoid speculative universal CRUD/key-value abstractions; model real capabilities.

## Authentication and authorization

- Authentication uses provider-neutral OAuth 2.0 and OpenID Connect.
- Interactive flows use Authorization Code with PKCE; services use workload identity or an explicitly approved machine flow.
- Validate issuer, audience, signature, expiry, key rotation and clock skew.
- Authorization follows a Zanzibar-style relationship model and is deny-by-default.
- Every protected application operation declares subject, resource, permission and required consistency.
- RBAC is expressed through relations/groups rather than handler conditionals.
- Tenant isolation is structural and must have cross-tenant negative tests.
- Use SpiceDB ZedTokens or another explicit consistency mechanism for causal/read-after-write sensitive workflows.
- Authorization unavailability must never silently become allow.
- Authorization schemas are versioned, migrated, tested and rollback-aware.

## Contract evolution and consistency

- Public REST, gRPC, event and serialized contracts require explicit versioning and compatibility rules.
- Breaking changes require an accepted RFC, migration plan and deprecation lifecycle.
- Every state-changing operation declares concurrency and idempotency semantics.
- Handle duplicate requests/messages, retries and out-of-order delivery explicitly.
- Use outbox/inbox or another accepted reliable-publication mechanism where needed.
- Do not claim exactly-once semantics without evidence.

## Database evolution

- Migrations are ordered, immutable and forward-only.
- Never edit, reorder or reuse a migration that may have reached a shared environment.
- Corrections require a new migration.
- Breaking changes use expand/migrate/contract and support rolling deployments.
- Long backfills are separate from short schema migrations.
- Production migration is an explicit controlled deployment step or job.
- Clean install, upgrade, drift and compatibility tests are mandatory.
- Destructive changes require a reviewed compatibility and recovery plan.

## Testing and coverage

- TDD red/green/refactor is the default for production behavior.
- Production Rust maintains 100% line and branch coverage.
- Coverage never replaces meaningful negative, boundary, property, contract, integration and end-to-end tests.
- Every rule, operation, adapter, serializer, error branch and migration path has appropriate tests.
- Exclusions are narrow, explicit, reviewed and recorded.
- Critical commercial and lifecycle rules use mutation-oriented evidence when practical.
- Tests and fixtures are deterministic and isolated.

## Self-provisioned environments

- CI provisions every database, cache, storage, authorization service, broker or external dependency required by tests.
- No manually prepared or shared long-lived test infrastructure.
- Prefer Rust Testcontainers for test-owned dependencies and Docker Compose for multi-service/E2E environments.
- GitHub service containers are for simple job-wide dependencies only.
- Pin container versions; `latest` is forbidden.
- Use protocol readiness probes, never fixed sleeps.
- Isolate databases, schemas, buckets, namespaces, queues, authorization stores and credentials per test/job.
- Run real migrations before integration tests.
- Collect sanitized logs and diagnostics on failure and tear resources down automatically.
- Local and GitHub Actions executions use the same versioned definitions.

## Documentation and examples

- Production modules, traits, structs, enums, functions, methods and non-obvious fields require clear current rustdoc.
- Document purpose, invariants, input, output, errors, side effects, transactions, idempotency, concurrency and security implications.
- Missing required documentation fails CI.
- Every external DTO/contract has canonical, realistic, pretty-printed, human-readable fixtures.
- Reuse fixtures in serialization, adapter, rustdoc, OpenAPI and compatibility tests.
- CI parses and validates every fixture; contract changes update docs and fixtures in the same change.

## Reliability, errors and diagnostics

- No swallowed errors, panic-based control flow or opaque generic failures in production paths.
- Errors are typed, contextual, causally chained and consistently mapped at boundaries.
- Structured logs, traces and metrics use correlation, causation and idempotency identifiers where relevant.
- Secrets, credentials, personal data and sensitive payloads are redacted by construction.
- Retry, timeout, circuit breaker, backpressure and degraded behavior are explicit and tested.
- Qualifying CI/runtime failures produce sanitized reproducibility bundles.
- Automatic GitHub issues require a stable fingerprint, exact reproduction, fixture, error chain, affected commit/release, owner, impact, acceptance criteria and verification evidence.
- Identical failures update one issue rather than creating duplicates.

## Performance engineering

- Performance claims require representative benchmarks and profiles.
- Critical operations have approved p50/p95/p99 latency, throughput, CPU, memory, allocation, I/O and database budgets.
- Benchmark before optimization, profile the bottleneck, change deliberately and compare against the baseline.
- Regression gates use controlled baselines and noise tolerance.
- N+1 queries, unbounded scans, hidden queries in loops and in-memory pagination of unbounded data are forbidden.
- Critical queries require bounded result shape, query-count assertions and plan evidence.
- Indexes are justified against workload and write cost.
- Transactions are minimal and observable; pools and timeouts are load-tested.
- Helm requests/limits derive from measurements, load, soak and saturation tests.

## Configuration, data and operations

- Configuration is typed, validated before traffic, auditable and example-backed.
- Secrets are injected and rotated; they never appear in committed files, logs or fixtures.
- Feature flags require owner, expiry, removal plan and supported-combination validation.
- Every data class has owner, classification, retention and allowed residency.
- Tenant offboarding covers primary data, derived data, caches, objects, logs and backups with verifiable erasure semantics.
- Critical services define SLI/SLO, alerts, runbooks, RPO/RTO, backup and automated restore tests.
- Rollback and disaster-recovery procedures must remain compatible with schema evolution.

## Engineering rules

- Keep `uc-domain` free from HTTP, database, messaging, identity and framework dependencies.
- Express invariants through types and domain methods; never use floating point for money.
- Document dependency purpose and maintenance implications.
- Every bug fix requires a regression test.
- Avoid `unwrap`, `expect` and panics in production paths.
- Unsafe Rust is forbidden without an accepted ADR.
- Avoid duplicate modules, abandoned scaffolding, parallel implementations and dead compatibility layers.
- Remove obsolete code/files in the same change unless a documented migration window requires coexistence.

## GitHub governance

- `governance/github-manifest.json` is the source of truth for labels, milestones, managed issues and Project #4.
- Do not create persistent GitHub metadata manually.
- Every managed issue is declared in the manifest; undefined metadata is removed in confirmed apply mode.
- No feature implementation resumes before issue #19 is approved.

## Release and packaging

- Release Please is the only authority that calculates and writes versions.
- Cargo packages, binaries, container images, Helm charts and GitHub Releases use one coordinated SemVer.
- Publishing derives from the immutable Release Please tag; existing tags/artifacts are never overwritten.
- Release topology, registries, signing or independent versions require an ADR/RFC.

## Validation

Run before proposing code changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Also run all applicable rustdoc, coverage, context, architecture, compatibility, authorization, adapter contract, fixture, migration, integration, failure-path, security and performance gates.

## Architecture changes

Create or update a decision record when changing boundaries, persistence, cache, storage, identity, authorization, consistency, event delivery, contracts, security, deployment, release, testing, documentation, operations, performance, migrations or the agentic operating model. Substantial or high-cost changes require an RFC before implementation.
