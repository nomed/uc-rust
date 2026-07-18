# ADR-0031 — Market Evidence Disposition and Scope Control

- Status: Accepted
- Date: 2026-07-19
- Governing issue: #41
- Source assessment: `.context/research/2025-market-guide-impact-assessment.md`
- Source revision: `nomed/uc-bok@9f0e1700cb7dfe49d5d684bc64fc76d1042054b1`
- Related: PROJECT_CHARTER.md, ADR-0024, ADR-0029, ADR-0030, #12, #38, #53

## Context

External market research is useful evidence for prioritization and architecture validation, but it is neither a normative specification nor an automatic product backlog. UC-BoK remains the normative semantic source and the accepted Project Charter governs UC Rust product scope.

Without an explicit disposition model, analyst features can enter the roadmap without ownership, acceptance evidence or compatibility with the canonical Capability → Operation → Capability Realization model.

## Decision

Every material market concern SHALL receive one explicit disposition:

- `adopted` — required architectural or product characteristic;
- `integrated` — supported through a governed external Capability Realization or adapter rather than implemented as core semantics;
- `planned` — accepted future capability with owner and release intent;
- `deferred` — intentionally outside the current release scope, with rationale;
- `rejected` — incompatible with project principles or unjustified by evidence.

No market item becomes roadmap scope unless it has:

1. accountable owner;
2. rationale and target release or deferral horizon;
3. dependencies and authority boundary;
4. UC-BoK mapping or explicit provisional status;
5. measurable acceptance evidence;
6. compatibility, security, offline and economic implications.

## Accepted dispositions for the assessed guide

| Market concern | Disposition | 1.0 position |
|---|---|---|
| POS as modular node | adopted | structural principle |
| Same source and behavior across endpoints | adopted | mandatory through canonical Operations |
| API-first/headless delivery | adopted | mandatory adapter/contract principle |
| Central and edge execution | adopted | required proving journey |
| Associate/customer mobility | planned | candidate profile; no full superapp |
| OMS and store fulfillment | integrated | contracts and delegated realization; full OMS deferred |
| Clienteling and loyalty | planned | governed capabilities outside the 1.0 golden path |
| Smart checkout, RFID, CV and IoT | integrated | governed extension/device boundaries |
| Offline payment | integrated | only with method-, provider- and country-specific evidence |
| Analytics and operational dashboards | integrated | derived/read-model and operational evidence surfaces |
| Embedded and agentic AI | integrated | human oversight, evaluation, audit and fallback required |
| Development toolkit and extensibility | adopted | governed registration and contracts; arbitrary plugins rejected |
| SoftPOS | deferred | candidate profile, not a 1.0 commitment |
| Retail media and influencer tracking | deferred | outside initial reference scope |
| Disaster recovery, privacy, security and performance | adopted | P0 product qualities |

## Scope boundaries

- Market terminology cannot override UC-BoK semantics.
- A listed feature is not evidence that UC Rust must implement it natively.
- Integration does not weaken authority, failure, compatibility, security, offline or economic obligations.
- “Same source” means shared canonical behavior, not one deployment artifact for every device and environment.
- “Headless” does not permit duplicated channel-specific business procedures.
- AI cannot bypass authorization, invariants, audit or human-control policy.
- A generic offline-payment claim is forbidden.
- A superapp UI remains outside the application/domain core.
- MACH does not imply mandatory premature microservice decomposition.

## Change and reassessment rule

A new guide revision or materially different external evidence creates a new impact assessment. It does not silently replace this baseline. Material scope changes follow the Project Charter change-control process and update roadmap ownership, UC-BoK traceability, budgets and release evidence.

## Consequences

- External evidence informs the project without becoming an uncontrolled requirements source.
- The 1.0 golden path remains coherent and bounded.
- Build, integrate and defer decisions remain explicit and auditable.
- Future market changes can be assessed incrementally against a pinned baseline.

## Rejected alternatives

- importing every analyst feature into the backlog;
- treating market research as normative over UC-BoK;
- leaving items as vague “future opportunities” without disposition;
- claiming integration without authority and failure semantics;
- silently updating the assessed source revision.

## Evidence

- `PROJECT_CHARTER.md`, especially Market evidence dispositions and non-goals;
- `docs/architecture/target-architecture.md`;
- `.context/research/2025-market-guide-impact-assessment.md`;
- `docs/roadmap/uc-rust-1.0-scope-and-traceability.md`;
- accepted ADR-0024, ADR-0029 and ADR-0030.

This ADR may be accepted before every planned or integrated capability has executable implementation evidence. Release gates must not claim implementation merely from this disposition record.