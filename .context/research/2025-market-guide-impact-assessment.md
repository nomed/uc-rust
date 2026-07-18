# 2025 Market Guide impact assessment

- Source repository: `nomed/uc-bok`
- Source path: `refs/2025_Market Guide for Unified Commerce Platforms Anchored by AI-Enabled POS for Tier 2 Retailers.md`
- Source revision: `9f0e1700cb7dfe49d5d684bc64fc76d1042054b1`
- Assessment date: 2026-07-18
- Related UC Rust issues: #11, #12, #29, #32, #38, #39, #40, #41

## Purpose

Use the market research as external evidence for product and architecture decisions without treating every market feature as a mandatory implementation requirement. UC-BoK remains the normative semantic source; UC Rust remains the reference implementation and executable validation environment.

## Confirmed structural directions

The guide reinforces the following UC Rust decisions:

1. POS is one modular intelligent node in a broader Unified Commerce platform rather than the owner of all commerce logic.
2. Commerce capabilities must be API-first, composable and headless so multiple touchpoints can reuse the same semantic operations.
3. The same source code and business behavior should be reused across fixed, mobile, self-checkout, central and edge endpoints.
4. Central orchestration must coexist with resilient edge execution and offline operation.
5. OMS, customer, loyalty, inventory, pricing, promotion and analytics capabilities must interoperate through explicit contracts.
6. Mobility is a first-class execution surface for associates and customers.
7. Smart checkout and IoT/RFID integration are important extension points rather than reasons to move business logic into device-specific code.
8. AI should be a platform capability with human oversight, evidence, security and performance controls—not scattered opaque automation.
9. Disaster recovery, security, privacy, availability and performance are product requirements, not infrastructure afterthoughts.
10. Retail-operational health is more important than process liveness alone.

## Capability impacts

| Market concern | UC Rust disposition | Governing work |
|---|---|---|
| Modular POS / POS as a node | Adopted architectural principle | ADR-0005, #22 |
| Same source code across endpoints | Adopted architectural principle | ADR-0005, ADR-0015, #22, #39 |
| API-first and headless | Required contract and adapter principle | #12, #22, #30 |
| Mobile POS and associate mobility | Candidate first-class delivery profile | #11, #12, #41 |
| OMS and flexible fulfillment | Core future capability, not part of first Basket slice | #9, roadmap |
| Clienteling and loyalty | Planned UC capability; offline guarantees must be explicit | #12, #39, #41 |
| Smart checkout / RFID / IoT | Extension and integration capability; device adapters remain replaceable | #12, #24, #41 |
| Offline payment | Capability-specific, risk-governed and provider-dependent | #31, #36, #39, #41 |
| Analytics and real-time dashboards | Read-model/observability concern; not domain authority | #12, #28, #32, #41 |
| Embedded and agentic AI | Platform capability with human-in-the-loop and evidence requirements | #16, #28, #29, #41 |
| Development toolkit / extensibility | Requires governed extension model, SDK and contract boundaries | #12, #30, #41 |
| Disaster recovery and resilience | Already P0 | #32 |
| Security/privacy/payment isolation | Already P0; payment data should remain outside core where possible | #16, #34, #36 |
| Retail media and influencer tracking | Not initial core scope; evaluate later through UC-BoK and product roadmap | #11, #38, #41 |
| SoftPOS | Candidate deployment profile/use case, not assumed core | #11, #12, #40, #41 |

## Required project decisions

The Project Charter and Target Architecture must explicitly decide:

- target retail segments for the reference implementation;
- first supported touchpoints and deployment profiles;
- which capabilities are implemented, integrated, demonstrated or intentionally deferred;
- extension model for retailer/vendor-built capabilities;
- AI capability boundaries and human-approval requirements;
- payment responsibility boundary and offline-payment risk model;
- mobility and associate-experience scope;
- relationship between global source of truth, central orchestration and edge authority.

## Non-goals derived from the assessment

- UC Rust will not implement every feature listed by a market analyst before delivering a coherent vertical slice.
- MACH will not be interpreted as mandatory microservice decomposition.
- AI will not be inserted into a workflow without measurable business value, reproducible evaluation and safe fallback.
- Headless will not mean semantic duplication across channels.
- Offline payment will not be claimed generically; each provider and payment method requires explicit evidence.
- A superapp UI is not part of the domain or application core.

## Evidence and follow-up

- Create and maintain #41 as the market-evidence disposition issue.
- Reference this assessment from the charter and target architecture.
- Map accepted UC-BoK identifiers into `governance/uc-bok-traceability.yaml` once UC-BoK dispositions are stable.
- Reassess on a new guide revision or material UC-BoK change.
