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

## Context is part of completion

- `.context/` is the durable operating memory for humans and agents.
- Maintaining `.context/` is mandatory work, not optional documentation.
- Every meaningful change must declare its context impact.
- Update durable records when architecture, domain ownership, public contracts, security, release behavior, database schema, governance or accepted assumptions change.
- Meaningful work requires a session record.
- Incomplete or delegated work requires a handoff.
- Sessions and handoffs preserve continuity and evidence but are not architecture authority.
- Material decisions must be promoted into numbered ADRs or RFCs rather than left only in a session, issue comment or chat.
- Agents may draft ADRs and RFCs; human approval is required to mark material decisions accepted.
- Accepted records are immutable. Replace them only through a new record that marks the old one superseded.
- When no context update is needed, record a concrete `no context impact` justification.
- Do not consider work complete while code and authoritative context disagree.
- Never store secrets, private chain-of-thought or unredacted sensitive data in context records.

## Single application core

- UC Rust uses hexagonal / ports-and-adapters architecture with use-case-oriented application operations.
- Every business operation has one canonical implementation.
- Domain types own invariants and state transitions.
- Application operations own use-case orchestration, transaction boundaries, idempotency coordination and process flow.
- REST, gRPC, messaging consumers, workers, scheduled jobs, CLI and batch processes are inbound adapters only.
- Inbound adapters may authenticate, validate transport syntax, map DTOs, establish technical context, invoke an application operation and map its result.
- Inbound adapters must not contain pricing rules, eligibility rules, lifecycle decisions, business workflow procedures or duplicated business branching.
- A worker or job must invoke the same application operation used by interactive adapters.
- Infrastructure implementations remain behind outbound ports.
- Shared business behavior must not be hidden in generic utility modules.
- Exceptions require an accepted ADR or RFC.

## Replaceable infrastructure adapters

- Domain and application crates depend on capability-oriented outbound ports, never provider SDKs.
- PostgreSQL, SQLite and future databases are persistence adapters.
- Redis, in-memory caches and future cache products are cache adapters.
- S3-compatible storage, cloud object stores and local filesystems are blob/storage adapters.
- SQL types, query builders, connection handles, Redis types, S3 SDK types, bucket names and filesystem paths must not leak into application contracts.
- Provider selection happens in the composition root through configuration and dependency injection.
- Ports must state required guarantees such as transactions, conditional writes, consistency, ordering, TTL, invalidation, streaming, range reads and idempotency.
- An adapter that cannot satisfy a required guarantee must fail capability validation explicitly; it must not silently weaken behavior.
- Every adapter implementation must pass the same reusable behavioral contract suite.
- Provider-specific optimization is allowed only behind the port.
- Engine-specific database migrations remain infrastructure details and must not alter business contracts.
- Do not create speculative universal CRUD, key-value or storage abstractions; define ports around real application capabilities.

## Database evolution

- Database migrations are ordered, immutable and forward-only.
- Never edit, reorder or reuse a migration that may have been applied to a shared environment.
- Corrections require a new migration.
- Breaking changes use expand/migrate/contract.
- Schema changes must remain compatible with rolling application deployments.
- Long-running backfills are separated from short schema migrations.
- Production migration execution is an explicit deployment step or dedicated controlled job, not an uncontrolled race between application instances.
- Clean-install, upgrade-path, drift and compatibility tests are required.
- Destructive schema changes require a reviewed compatibility plan.
- Database patches must not hide application business logic.

## Testing and coverage

- Test-driven development using red, green and refactor is the default workflow for production behavior.
- Production Rust code must maintain 100% line and branch coverage.
- Coverage is mandatory but never replaces meaningful assertions, negative cases, boundary cases, property tests, contract tests, integration tests and end-to-end verification.
- Every business rule, application operation, adapter behavior, serializer, error branch and migration path requires appropriate tests.
- Generated code or genuinely unreachable defensive paths may be excluded only through an explicit, reviewed and narrowly scoped policy.
- Critical commercial and lifecycle rules should use mutation testing or equivalent evidence when practical.
- Tests and fixtures must be deterministic, isolated and free from timing, ordering or external-environment assumptions.
- A change is not complete if it adds production behavior without corresponding tests.

## Documentation and executable examples

- Production modules, traits, structs, enums, functions, methods and non-obvious fields require clear and current rustdoc.
- Documentation must explain purpose, invariants, inputs, outputs, errors, side effects, transaction behavior, idempotency, concurrency assumptions and security implications where relevant.
- Missing required documentation must fail CI.
- Every external DTO and serialized contract requires canonical, realistic and human-readable examples.
- Prefer pretty-printed versioned fixture files for JSON payloads and equivalent readable formats for other serializations.
- Reuse canonical fixtures in serialization/deserialization tests, adapter tests, rustdoc, OpenAPI or other contract documentation and compatibility tests where practical.
- CI must parse and validate fixture files so examples cannot silently become stale.
- Round-trip tests should deserialize canonical input, exercise the relevant behavior, serialize output and validate semantic content.
- Snapshot or golden tests may support readability but must not replace explicit assertions for important behavior.
- Contract and behavior changes require documentation and fixture updates in the same change.

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
- Avoid duplicate modules, abandoned scaffolding, parallel implementations and dead compatibility layers.
- Remove obsolete code and files in the same change that replaces them, unless a documented migration window requires coexistence.

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

When applicable, also run rustdoc linting, 100% line/branch coverage, context validation, architecture tests, adapter contract suites, fixture round-trip tests, migration clean-install tests and supported upgrade-path tests.

## Architecture changes

Create or update a decision record when changing boundaries, persistence strategy, cache or storage contracts, event delivery, public contracts, security model, deployment topology, release model, application operation ownership, testing policy, documentation policy, database migration policy or agentic operating model. Substantial or high-cost changes require an RFC before implementation.