# 2025 Market Guide impact assessment

- Status: Reviewable for acceptance
- Governing issue: #41
- Governing decision: ADR-0031
- Source repository: `nomed/uc-bok`
- Source path: `refs/2025_Market Guide for Unified Commerce Platforms Anchored by AI-Enabled POS for Tier 2 Retailers.md`
- Source revision: `9f0e1700cb7dfe49d5d684bc64fc76d1042054b1`
- Assessment date: 2026-07-18
- Disposition completed: 2026-07-19
- Related UC Rust issues: #11, #12, #29, #32, #38, #39, #40, #41, #53

## Purpose

Use market research as external evidence for product and architecture decisions without treating every listed feature as a mandatory implementation requirement. UC-BoK remains the normative semantic source; UC Rust remains the reference implementation and executable validation environment; the accepted Project Charter governs product scope.

## Disposition vocabulary

Each material concern is classified as one of:

- `adopted` — required architecture or product characteristic;
- `integrated` — supplied through a governed external realization, adapter or read-model surface;
- `planned` — accepted future capability with explicit roadmap ownership;
- `deferred` — intentionally outside the current release scope;
- `rejected` — inconsistent with project principles or unsupported by sufficient evidence.

Absence of a disposition never implies scope.

## Confirmed structural directions

The guide reinforces the following UC Rust decisions:

1. POS is one modular intelligent node in a broader Unified Commerce platform rather than the owner of all commerce logic.
2. Commerce capabilities are API-first, composable and headless so multiple touchpoints reuse the same canonical Operations.
3. Fixed, mobile, self-checkout, central, edge, worker and CLI surfaces reuse one canonical business behavior.
4. Central orchestration coexists with resilient edge execution and explicitly governed offline operation.
5. OMS, customer, loyalty, inventory, pricing, promotion and analytics interoperate through explicit contracts and Capability Realizations.
6. Mobility is a first-class candidate execution surface for associates and customers, without making a superapp part of the application core.
7. Smart checkout and IoT/RFID/CV are governed extension and integration points rather than reasons to move business logic into device-specific code.
8. AI is a governed capability with human oversight, evidence, security, audit and performance controls.
9. Disaster recovery, security, privacy, availability and performance are product requirements rather than infrastructure afterthoughts.
10. Retail-operational health is more important than process liveness alone.

## Final market dispositions

| Market concern | Disposition | 1.0 position | Governing evidence |
|---|---|---|---|
| Modular POS / POS as a node | adopted | structural architecture principle | Project Charter, target architecture, ADR-0021 |
| Same source and behavior across endpoints | adopted | mandatory canonical Operation reuse | ADR-0021, ADR-0027, ADR-0028 |
| API-first and headless | adopted | required contract and adapter principle | target architecture, ADR-0028 |
| Central and edge execution | adopted | required 1.0 proving journey | Project Charter, target architecture |
| Mobile POS and associate mobility | planned | candidate runtime/touchpoint; full superapp deferred | Project Charter, #53 |
| OMS and flexible fulfillment | integrated | explicit contracts and delegated realization; full enterprise OMS deferred | Project Charter, ADR-0024 |
| Clienteling and loyalty | planned | governed capabilities outside the 1.0 golden path | Project Charter, #53 |
| Smart checkout / RFID / CV / IoT | integrated | replaceable device and extension adapters | ADR-0029, target architecture |
| Offline payment | integrated | provider-, method- and country-specific only | Project Charter, ADR-0024 |
| Analytics and real-time dashboards | integrated | read-model and operational evidence concern; never transaction authority | ADR-0026, target architecture |
| Embedded and agentic AI | integrated | human oversight, evaluation, audit and safe fallback | Project Charter, target architecture |
| Development toolkit / extensibility | adopted | governed registration and public contracts | ADR-0029 |
| SoftPOS | deferred | candidate profile; not a 1.0 commitment | Project Charter, #53 |
| Retail media and influencer tracking | deferred | outside the initial reference scope | Project Charter |
| Disaster recovery and resilience | adopted | P0 quality obligation | Project Charter, quality model |
| Security, privacy and payment isolation | adopted | P0 structural boundary | Project Charter, target architecture |

## Roadmap admission rule

No market feature enters the roadmap without:

1. accountable owner;
2. rationale and target release or explicit deferral horizon;
3. dependencies and authority boundary;
4. UC-BoK mapping or explicit provisional status;
5. measurable acceptance evidence;
6. security, compatibility, offline, performance and economic implications.

A disposition of `integrated` does not remove the need to define canonical semantics, provider authority, failure behavior, version compatibility, idempotency, fallback, observability or cost attribution.

## Explicit non-goals and rejected interpretations

- UC Rust will not implement every feature listed by a market analyst before delivering a coherent vertical slice.
- Market research does not override UC-BoK normative meaning.
- MACH is not interpreted as mandatory microservice decomposition.
- AI is not inserted into a workflow without measurable value, reproducible evaluation, authorization, audit and safe fallback.
- Headless does not mean semantic duplication across channels.
- “Same source” means one canonical behavior, not necessarily one deployment artifact for every environment.
- Offline payment is never claimed generically.
- A customer or associate superapp UI is not part of the domain/application core.
- Device, provider or analyst terminology cannot leak into canonical business contracts without an accepted semantic decision.

## Traceability position

Stable UC-BoK mapping is governed by ADR-0030. Market concerns map to typed UC-BoK identifiers only when they resolve against the pinned normative manifest. Provisional labels remain visibly provisional and cannot be presented as stable specification identifiers.

This assessment records product disposition, not implementation status. `adopted`, `integrated` or `planned` must not be translated into `implemented` without qualifying code, contract and test evidence.

## Reassessment rule

The assessed source revision is immutable for this assessment. A new Market Guide revision, material external research update or material UC-BoK change creates a new impact assessment and explicit adoption decision; it does not silently replace this baseline.

## Completion evidence

- Project Charter explicitly references this assessment and records in-scope and non-goal decisions.
- The accepted target architecture contains the required structural impacts.
- The 1.0 roadmap distinguishes adopted, integrated, planned and deferred work.
- ADR-0030 establishes stable typed UC-BoK traceability rules.
- ADR-0031 governs market-evidence disposition and future reassessment.

The discovery and architectural disposition work is complete. Executable proof for individual capabilities remains governed by their release gates and cannot be inferred from this assessment alone.
