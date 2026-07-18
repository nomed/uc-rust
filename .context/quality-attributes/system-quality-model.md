# UC Rust system quality model

- Status: Defined baseline; enforcement in progress
- Governing issue: #37
- Gate consumer: #19
- Execution roadmap: `.context/quality-attributes/enforcement-roadmap.md`
- Last updated: 2026-07-18

This document is the authoritative completeness matrix for non-functional and governance requirements. A P0 quality attribute is not complete until it has an owner, measurable requirement, automated enforcement and reproducible evidence accepted by the Project Ready gate.

## Maturity scoring

- `Planned` = 0% — requirement is not yet complete enough to execute.
- `Defined` = 20% — owner, threshold, issue and enforcement plan exist.
- `Implemented` = 40% — repository control/test/tool exists.
- `Enforced` = 60% — failure blocks the relevant merge/release/deployment.
- `Evidenced` = 80% — clean, reproducible proof is linked.
- `Accepted` = 100% — Project Ready accepts the evidence or an approved exception.

Current baseline: **all 25 P0 rows are at least Defined; 7 rows are Implemented; maturity score 24.8%**. Implemented controls remain below Evidenced until a clean intentional run is observed and linked.

| Quality attribute | Requirement / invariant | Metric or threshold | Governing record / issue | Enforcement evidence | Owner | Status |
|---|---|---|---|---|---|---|
| Context freshness | Code and authoritative context never diverge | 100% meaningful PRs declare context impact | ADR-0004 / #21 | Context CI, session/ADR/RFC checks | Documentation & Knowledge Agent | Implemented |
| Architecture integrity | One canonical application core; adapters contain no business logic | Zero prohibited inward dependencies or duplicate operations | ADR-0005 / #22 | Architecture tests and code review gates | Domain Architect | Implemented |
| Replaceable infrastructure | DB, cache, storage and authorization providers remain behind stable ports | All adapters pass shared contract suites | ADR-0007, ADR-0013 / #24, #36 | Contract and architecture tests | Domain Architect | Implemented |
| UC-BoK specification alignment | UC Rust is traceably aligned to a declared UC-BoK revision and all divergences are governed | 100% implemented capabilities and public contracts map to stable UC-BoK identifiers; zero silent divergence | ADR-0014 / #38 / nomed/uc-bok#9 | Traceability manifest validation, drift report and reciprocal issue checks | Product Analyst & Domain Architect | Defined |
| Offline retail continuity | Selected retail capabilities remain safe and operational through WAN outage using the same canonical business operations | 100% critical capabilities have an offline classification, authority model and tested partition behavior; zero lost or duplicated business effects | ADR-0015, ADR-0018 / #39 / nomed/uc-bok#10 | Capability-profile validation, partition/chaos tests, sync contract tests and reconciliation evidence | Distributed Systems Architect | Defined |
| Edge synchronization and convergence | Central replicas and edge-authoritative data converge through durable, versioned and semantically governed synchronization | 100% synchronized data types declare authority, freshness, ordering, deduplication and conflict policy; zero generic silent last-write-wins | ADR-0015, ADR-0018 / #39 | Snapshot/delta, outbox/inbox, bootstrap, restart and re-sync suites | Sync & Data Agent | Implemented |
| Edge fleet control | Every peripheral node is securely inventoried, monitored and reconciled from desired to actual state | 100% managed edges have unique identity, current inventory and detectable drift | ADR-0016, ADR-0018 / #40 | Fleet inventory, identity, drift and reconciliation tests | Platform & Operations Agents | Defined |
| Edge deployment safety | Peripheral updates are signed, compatible, resumable, staged and rollback-capable without losing retail data | 100% artifacts verified; interrupted and failed updates preserve active service/data; canary pause/rollback tests pass | ADR-0016, ADR-0018 / #40 | Simulated fleet rollout, WAN interruption, signature, compatibility and rollback evidence | Release & Operations Agents | Defined |
| Database evolution | Migrations are immutable, forward-only and rolling-compatible | 100% clean install and supported upgrade paths pass | ADR-0006 / #23 | Migration CI and drift checks | Data/Infrastructure Agent | Defined |
| Correctness | Production Rust maintains complete line and branch coverage | 100% line and branch coverage | ADR-0008 / #25 | Coverage and mutation-oriented checks | Verification Agent | Implemented |
| Documentation | Production APIs and contracts are current and executable | Zero missing required rustdoc; all DTO fixtures validate | ADR-0009 / #26 | rustdoc lint and fixture round trips | Documentation & Knowledge Agent | Implemented |
| Reproducible tests | Tests provision all external services automatically | 100% required suites run on a clean runner | ADR-0010 / #27 | Testcontainers/Compose workflow | Verification Agent | Defined |
| Reliability | Failures are typed, safe, observable and reproducible | Zero swallowed errors; agent-ready bundle for qualifying failures | ADR-0011 / #28 | Failure-path tests, diagnostic artifacts, issue fingerprinting | Reliability Agent | Defined |
| Performance | Critical operations obey measured CPU, memory, latency, network and DB budgets | Per-operation p50/p95/p99, throughput, CPU, memory, I/O, query and bytes-transferred budgets | ADR-0012, ADR-0018 / #29 | Benchmarks, profiles, load and regression gates | Performance Agent | Defined |
| Cost efficiency | Architecture minimizes recurring cost-to-serve without weakening required guarantees | 100% material components have minimum topology, central/edge footprint, scaling unit, operational-cost rationale and a measured simpler alternative | ADR-0018 / #29, #39, #40 | Component scorecards, unit-cost benchmarks and architecture review gate | Product Architect & FinOps Owner | Defined |
| Contract evolution | Public contracts remain compatible or migrate explicitly | Zero unapproved breaking changes | #30 | Schema compatibility and consumer contract tests | API/Contract Owner | Defined |
| Consistency | Retries, concurrency, partitions and duplicate delivery are safe under an explicit per-data-class model | 100% synchronized data classes declare authority, consistency, ordering, idempotency, conflict and recovery semantics; consensus only through accepted exception | ADR-0018 / #31, #39 | Data-class manifest validation, concurrency, outbox/inbox and partition tests | Domain & Distributed Systems Architects | Defined |
| Operability | Services have SLOs, recovery and tested backups | Restore tests pass; RPO/RTO met | #32 | SLO checks, restore drills and runbooks | Operations Agent | Defined |
| Configuration | Configuration is typed, valid, auditable and cleaned up | Zero unowned or expired feature flags | #33 | Schema validation and flag lifecycle CI | Platform Agent | Defined |
| Data governance | Data lifecycle, privacy and tenant deletion are complete | 100% data classes have owner/retention/location | #34 | Erasure/export tests and classification validation | Security & Privacy Agent | Defined |
| Developer experience | Local and CI environments are pinned and reproducible | Clean bootstrap and all standard commands pass | #35 | Bootstrap/onboarding workflow | Golden Path Agent | Implemented |
| Authentication | Identity verification is standards-based and provider-neutral | 100% protected entry points validate issuer/audience/signature/expiry | ADR-0013 / #36 | OIDC integration and negative tests | Security Agent | Defined |
| Authorization | Zanzibar-style deny-by-default decisions protect every operation | 100% protected operations declare permission; zero cross-tenant leaks | ADR-0013 / #36 | SpiceDB model, contract, property and isolation tests | Security Agent | Defined |
| Supply chain | Dependencies and release artifacts are trusted and traceable | Zero unresolved prohibited licenses/vulnerabilities; signed release artifacts | #16 and release policy | cargo-deny/audit, SBOM, provenance and signing | Security & Release Agents | Defined |
| Release integrity | Cargo, containers, Helm and GitHub Release share one immutable version | 100% artifacts derive from Release Please tag | ADR-0003 / release policy | Release workflows and artifact verification | Release Agent | Defined |

## Completeness rule

The Project Ready gate cannot be approved while any P0 row is missing one of the following:

1. governing record or issue;
2. measurable threshold or invariant;
3. implemented automated control;
4. enforced failure behavior;
5. reproducible evidence artifact;
6. accountable owner;
7. explicit maturity status;
8. Project Ready acceptance or approved exception.

Temporary exceptions require a written reason, owner, expiry date and compensating control. Expired exceptions fail the gate.
