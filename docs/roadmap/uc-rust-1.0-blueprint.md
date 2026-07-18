# UC Rust — Architecture and Delivery Blueprint to 1.0

- Status: Proposed for accountable review
- Date: 2026-07-18
- Governing epic: #45
- Planning baseline: `docs/roadmap/uc-rust-1.0-scope-and-traceability.md`
- Normative sources: UC-BoK and Economics by Design

## 1.0 product statement

UC Rust 1.0 is a production-grade, model-driven Unified Commerce reference platform that proves a canonical set of retail operations can run consistently in central and store-edge profiles, remain operational through supported WAN failures, converge data safely, be deployed and observed as a managed fleet, and expose measurable unit economics.

It is not intended to implement every retail capability by 1.0. It establishes the architectural kernel and a coherent end-to-end commercial transaction path that future capabilities can extend without changing foundational contracts.

## 1.0 proving journey

```text
Product/catalog reference
 -> Basket
 -> Pricing and promotion calculation
 -> Checkout
 -> Payment/fiscal provider evidence boundaries
 -> Order
 -> durable publication
 -> edge/central convergence
 -> fleet-managed deployment
 -> operation-level economic evidence
```

The governed capability ownership, release allocation, profile/offline classes, authority, quality and economic evidence are defined in `uc-rust-1.0-scope-and-traceability.md`. Capabilities absent from that baseline are not implicit 1.0 requirements.

## Release train

### M0 — Project Ready

Purpose: approve scope, architecture, governance and enforcement model.

Exit:
- charter and 1.0 scope accepted;
- target architecture and plane boundaries accepted;
- quality model fully defined;
- UC-BoK and EbD traceability governed;
- Yukh migration is controlled and non-blocking;
- release roadmap and review gates approved.

### M1 — Runtime Foundation

Purpose: create the Operation First execution kernel.

Deliverables:
- operation model and invocation context;
- lifecycle and graceful shutdown;
- explicit composition/configuration;
- error and cancellation model;
- observability/health/economic correlation;
- scheduler/worker contracts;
- delivery adapter contracts;
- extension boundaries;
- first multi-adapter operation proof.

Exit:
- accepted runtime RFC and child ADRs;
- central and edge profiles compose the same operation;
- architecture, performance and cost evidence.

### M2 — Persistent Basket

Purpose: first complete domain/application vertical slice.

Deliverables:
- basket aggregate and operations;
- SQLite and PostgreSQL adapters behind one port;
- REST and one second adapter;
- canonical fixtures;
- authorization checks;
- durable local persistence;
- operation economics report.

Exit:
- same tests pass across persistence adapters;
- no business behavior in adapters;
- 100% governed coverage and documentation evidence.

### M3 — Commercial Basket

Purpose: pricing and promotion ownership with reproducible commercial evidence.

Deliverables:
- product/catalog reference contract;
- price and promotion inputs;
- commercial calculation operations;
- immutable calculation snapshot/explanation;
- offline data/freshness policy;
- economic and performance baseline.

Exit:
- deterministic commercial result from canonical fixtures;
- edge behavior and staleness limits proven;
- pricing remains a replaceable capability.

### M4 — Checkout and Order

Purpose: turn a commercial basket into durable retail outcomes.

Deliverables:
- checkout state machine;
- payment and fiscal provider boundaries;
- order creation;
- idempotency and compensation policy;
- outbox and reliable event publication;
- audit evidence.

Exit:
- retries cannot duplicate business effects;
- failures are recoverable and reproducible;
- completed transaction produces an Order and durable publication evidence.

### M5 — Distributed Store Edge

Purpose: prove WAN-independent retail continuity and convergence.

Deliverables:
- capability/data authority manifests;
- central-to-edge snapshots/deltas;
- durable edge-to-central sync;
- restart, partition, duplicate, reorder and re-sync harnesses;
- offline authorization policy;
- central/edge compatibility matrix.

Exit:
- zero lost or silently duplicated effects;
- no silent conflict resolution;
- supported offline journey completes and converges.

### M6 — Fleet Control and Safe Delivery

Purpose: manage store-edge as an observable and safely deployable fleet.

Deliverables:
- unique edge identity and inventory;
- desired/actual state;
- signed release/compatibility manifest;
- pull-based staged update;
- canary, pause and rollback;
- technical and retail-operational health;
- configuration revision and drift detection.

Exit:
- interrupted update preserves service and data;
- rollout failure pauses or rolls back;
- every managed edge is inventoried and attributable.

### M7 — Identity, Security and Data Governance

Purpose: make security guarantees operational rather than architectural prose.

Deliverables:
- OIDC authentication adapter and test identity provider;
- Zanzibar-compatible authorization adapter and model;
- cross-tenant isolation evidence;
- secrets, supply chain, SBOM, signing and provenance;
- retention/export/erasure/offboarding contracts;
- threat model and security failure tests.

Exit:
- deny-by-default and tenant isolation proven;
- release artifacts and edge updates verifiable;
- no unresolved P0 threat.

### M8 — Operational and Economic Readiness

Purpose: prove reliability, recovery, performance and cost-to-serve.

Deliverables:
- SLI/SLO and alert model;
- backup/restore and disaster-recovery exercises;
- runbooks and actionable failure bundles;
- load/stress/soak baselines;
- direct technical cost per economic unit;
- store/day and tenant/month projections;
- regression gates and capacity model.

Exit:
- RPO/RTO evidence;
- accepted performance and resource budgets;
- reproducible economic reports and no unexplained regression.

### RC — 1.0 Release Candidate

Purpose: freeze public contracts and validate the full reference journey.

Exit:
- contract compatibility matrix complete;
- no open P0/P1 release blockers;
- installation, upgrade, rollback and recovery rehearsed;
- UC-BoK coverage/drift report accepted;
- release artifacts produced through release-please.

### 1.0 — Reference Release

Release content:
- central and store-edge runtime profiles;
- canonical basket-to-order transaction path;
- offline continuity and convergence proof;
- fleet-managed signed deployment;
- OIDC and Zanzibar-compatible authorization boundaries;
- operational and economic evidence;
- documented extension contracts.

## Cross-release workstreams

The following never become an afterthought:

- architecture integrity and one-place business logic;
- documentation, fixtures and TDD;
- security and tenant isolation;
- performance and memory efficiency;
- Economics by Design;
- UC-BoK traceability;
- agentic context freshness;
- compatibility and migrations;
- failure reproduction and actionable issues.

## Planning rule

No release enters implementation until its epic contains:

- accepted scope and non-goals;
- architecture records for unresolved decisions;
- child issues with dependencies and evidence;
- compatibility and migration impact;
- performance and economic budgets;
- failure and rollback proof;
- explicit exit gate.

## Change control

The roadmap is a governed baseline, not an immutable promise. Material changes require:

- impact assessment on 1.0 scope;
- ADR/RFC update where architectural;
- UC-BoK and EbD impact disposition;
- project roadmap, matrix and dependency update;
- named owner and measurable exit evidence;
- explicit acceptance rather than silent scope drift.

The detailed change-control contract, deferred capabilities and compatibility expectations are normative in `uc-rust-1.0-scope-and-traceability.md`.