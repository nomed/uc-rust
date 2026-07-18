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
7. `governance/uc-bok-traceability.yaml`
8. the governing GitHub issue and latest applicable handoff
9. project charter, target architecture, affected public APIs and tests

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

## UC-BoK reference implementation

- `nomed/uc-bok` is normative for Unified Commerce concepts, principles, capabilities, business objects, relationships, constraints and schemas.
- UC Rust is the reference implementation and executable validation environment.
- Every implemented capability, domain type, application operation, public contract and critical test maps to stable UC-BoK identifiers where such identifiers exist.
- Every meaningful change declares UC-BoK impact: none, implements-existing, clarification-required, specification-defect, extension-proposed, adoption-required or intentional-divergence.
- Update `governance/uc-bok-traceability.yaml` when implementation status or mapping changes.
- New UC-BoK normative material requires explicit UC Rust impact assessment; implementation findings that challenge the model require a reciprocal UC-BoK issue.
- Silent divergence is forbidden. Intentional divergence requires accepted records in both repositories and explicit compatibility consequences.

## Single application core

- Use hexagonal / ports-and-adapters architecture with use-case-oriented application operations.
- Every business operation has one canonical implementation.
- Domain types own invariants and state transitions.
- Application operations own orchestration, transactions, idempotency and process flow.
- REST, gRPC, consumers, workers, jobs, CLI and batch processes are inbound adapters only.
- Inbound adapters may authenticate, validate transport syntax, map DTOs, establish technical context, invoke an operation and map the result.
- Adapters must not contain business workflows, pricing, eligibility, lifecycle decisions or duplicated branching.
- Infrastructure remains behind outbound ports; generic utilities must not hide business behavior.

## Distributed retail runtime and offline operation

- UC Rust supports central, store-edge, warehouse-edge and future declared deployment profiles built from the same domain model and application operations.
- Edge is an autonomous runtime profile, not a passive cache and not a separately reimplemented product.
- Every retail capability declares exactly one offline mode: `offline-capable`, `offline-capable-with-limits`, `read-only-offline`, `degraded-offline`, `online-required` or `forbidden-offline`.
- The same public operation must preserve the same business semantics at central and edge; unavailable capabilities are reported explicitly through capability discovery rather than silently changing meaning.
- Distinguish cache, replicated central data and edge-authoritative local data. They must not share an ambiguous lifecycle or authority model.
- Every synchronized data class declares authority, freshness, consistency, retention, conflict policy and recovery behavior.
- Generic last-write-wins conflict resolution is forbidden unless explicitly justified for that data class through an accepted decision.
- Edge-authoritative business effects must survive WAN outage, process restart, update interruption and full re-sync.
- Sync is a first-class platform capability with durable checkpoints, source identity, sequence, event identity, idempotency, retry, ordering, deduplication, reconciliation and version negotiation.
- Central-to-edge snapshots/deltas and edge-to-central outbox/inbox flows are distinct and must be tested independently.
- Bootstrap, re-bootstrap and full re-sync must preserve unsent local work and produce reproducible evidence.
- Tests must cover WAN partition, reconnect, duplicate delivery, delayed delivery, out-of-order delivery, restart, partial failure and semantic conflict handling.
- Offline authentication and authorization guarantees, revocation exposure and high-risk online-only operations must be explicit and tested.
- Supported central version, edge version, DB schema, sync protocol, contract version, configuration schema, authorization schema and UC-BoK revision combinations are machine-validated.

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

## Edge fleet control and deployment

- The central control plane owns desired state; every edge owns safe local reconciliation and reports actual state.
- Edge update is pull-based so intermittent WAN cannot make successful rollout depend on a fragile central push session.
- Every edge has a unique device identity bound to tenant/location and uses authenticated, authorized and auditable communication, preferably mTLS with rotation and revocation.
- Fleet inventory includes hardware, OS, architecture, software versions, DB schema, sync protocol, configuration revision, authorization schema, UC-BoK revision, capability profile and last-seen state.
- Release manifests and artifacts are immutable, content-addressed, signed and provenance/SBOM backed. Unsigned, corrupted, incompatible, revoked or unauthorized downgrade artifacts are rejected locally.
- Software release, configuration revision, DB schema, sync protocol and authorization model may version independently but must pass one compatibility-envelope validation before activation.
- Updates require preflight validation of compatibility, disk/memory, maintenance window, sync safety, local outbox durability and absence of critical retail operations.
- Installation is resumable and staged. Use atomic switch, dual-slot/A-B or an equivalent proven mechanism where appropriate.
- Post-update health verification decides commit or automatic rollback; application rollback must preserve business data and remain schema-compatible.
- Rollout supports lab, canary, pilot, cohort, percentage and fleet-wide stages with automatic pause/rollback thresholds.
- Technical health and retail-operational health are separate. A process may be running while the store cannot sell; an edge may be WAN-offline while remaining operational.
- Required retail health includes ability to sell, pricing/configuration validity, local DB health, durable outbox state and synchronization freshness/backlog.
- Desired/actual drift in software, configuration, schema, certificates, authorization model or deployed components is detected and actionable.
- Remote operations are typed, least-privilege, authenticated, authorized, expiring, idempotent and audited. Permanent unrestricted remote shell access is not a management feature.
- Tests must prove interruption safety, WAN loss during download/install, canary advancement, automatic pause, rollback, drift detection, certificate revocation and telemetry buffering.

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
- Edge and fleet suites must provision ephemeral central and edge nodes, controllable WAN partitions and update artifacts.

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
- Edge performance budgets include startup, local API latency, memory/CPU footprint, disk growth, sync throughput, backlog recovery time and operation under constrained hardware.

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
- Edge release manifests must identify the compatible central, DB, sync, contract, configuration, authorization and UC-BoK versions.

## Validation

Run before proposing code changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Also run all applicable rustdoc, coverage, context, UC-BoK traceability, architecture, compatibility, authorization, adapter contract, fixture, migration, integration, WAN-partition, synchronization, fleet-rollout, failure-path, security and performance gates.

## Architecture changes

Create or update a decision record when changing boundaries, persistence, cache, storage, identity, authorization, consistency, event delivery, contracts, security, central/edge profiles, synchronization, data authority, fleet management, deployment, release, testing, documentation, operations, performance, migrations or the agentic operating model. Substantial or high-cost changes require an RFC before implementation.
