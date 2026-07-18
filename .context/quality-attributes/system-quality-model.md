# UC Rust system quality model

- Status: Draft baseline
- Governing issue: #37
- Gate consumer: #19
- Last updated: 2026-07-18

This document is the authoritative completeness matrix for non-functional and governance requirements. A P0 quality attribute is not complete until it has an owner, measurable requirement, automated enforcement and reproducible evidence.

| Quality attribute | Requirement / invariant | Metric or threshold | Governing record / issue | Enforcement evidence | Owner | Status |
|---|---|---|---|---|---|---|
| Context freshness | Code and authoritative context never diverge | 100% meaningful PRs declare context impact | ADR-0004 / #21 | Context CI, session/ADR/RFC checks | Documentation & Knowledge Agent | Planned |
| Architecture integrity | One canonical application core; adapters contain no business logic | Zero prohibited inward dependencies or duplicate operations | ADR-0005 / #22 | Architecture tests and code review gates | Domain Architect | Planned |
| Replaceable infrastructure | DB, cache, storage and authorization providers remain behind stable ports | All adapters pass shared contract suites | ADR-0007, ADR-0013 / #24, #36 | Contract and architecture tests | Domain Architect | Planned |
| UC-BoK specification alignment | UC Rust is traceably aligned to a declared UC-BoK revision and all divergences are governed | 100% implemented capabilities and public contracts map to stable UC-BoK identifiers; zero silent divergence | ADR-0014 / #38 / nomed/uc-bok#9 | Traceability manifest validation, drift report and reciprocal issue checks | Product Analyst & Domain Architect | Planned |
| Database evolution | Migrations are immutable, forward-only and rolling-compatible | 100% clean install and supported upgrade paths pass | ADR-0006 / #23 | Migration CI and drift checks | Data/Infrastructure Agent | Planned |
| Correctness | Production Rust maintains complete line and branch coverage | 100% line and branch coverage | ADR-0008 / #25 | Coverage and mutation-oriented checks | Verification Agent | Planned |
| Documentation | Production APIs and contracts are current and executable | Zero missing required rustdoc; all DTO fixtures validate | ADR-0009 / #26 | rustdoc lint and fixture round trips | Documentation & Knowledge Agent | Planned |
| Reproducible tests | Tests provision all external services automatically | 100% required suites run on a clean runner | ADR-0010 / #27 | Testcontainers/Compose workflow | Verification Agent | Planned |
| Reliability | Failures are typed, safe, observable and reproducible | Zero swallowed errors; agent-ready bundle for qualifying failures | ADR-0011 / #28 | Failure-path tests, diagnostic artifacts, issue fingerprinting | Reliability Agent | Planned |
| Performance | Critical operations obey measured CPU, memory, latency and DB budgets | Per-operation p50/p95/p99, CPU, memory and query budgets | ADR-0012 / #29 | Benchmarks, profiles, load and regression gates | Performance Agent | Planned |
| Contract evolution | Public contracts remain compatible or migrate explicitly | Zero unapproved breaking changes | #30 | Schema compatibility and consumer contract tests | API/Contract Owner | Planned |
| Consistency | Retries, concurrency and duplicate delivery are safe | 100% state-changing operations declare idempotency/concurrency semantics | #31 | Concurrency, outbox/inbox and duplicate-delivery tests | Domain Architect | Planned |
| Operability | Services have SLOs, recovery and tested backups | Restore tests pass; RPO/RTO met | #32 | SLO checks, restore drills and runbooks | Operations Agent | Planned |
| Configuration | Configuration is typed, valid, auditable and cleaned up | Zero unowned or expired feature flags | #33 | Schema validation and flag lifecycle CI | Platform Agent | Planned |
| Data governance | Data lifecycle, privacy and tenant deletion are complete | 100% data classes have owner/retention/location | #34 | Erasure/export tests and classification validation | Security & Privacy Agent | Planned |
| Developer experience | Local and CI environments are pinned and reproducible | Clean bootstrap and all standard commands pass | #35 | Bootstrap/onboarding workflow | Golden Path Agent | Planned |
| Authentication | Identity verification is standards-based and provider-neutral | 100% protected entry points validate issuer/audience/signature/expiry | ADR-0013 / #36 | OIDC integration and negative tests | Security Agent | Planned |
| Authorization | Zanzibar-style deny-by-default decisions protect every operation | 100% protected operations declare permission; zero cross-tenant leaks | ADR-0013 / #36 | SpiceDB model, contract, property and isolation tests | Security Agent | Planned |
| Supply chain | Dependencies and release artifacts are trusted and traceable | Zero unresolved prohibited licenses/vulnerabilities; signed release artifacts | #16 and release policy | cargo-deny/audit, SBOM, provenance and signing | Security & Release Agents | Planned |
| Release integrity | Cargo, containers, Helm and GitHub Release share one immutable version | 100% artifacts derive from Release Please tag | ADR-0003 / release policy | Release workflows and artifact verification | Release Agent | Planned |

## Completeness rule

The Project Ready gate cannot be approved while any P0 row is missing one of the following:

1. governing record or issue;
2. measurable threshold or invariant;
3. automated enforcement plan;
4. required evidence artifact;
5. accountable owner;
6. explicit status.

Temporary exceptions require a written reason, owner, expiry date and compensating control. Expired exceptions fail the gate.
