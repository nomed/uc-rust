# UC Rust 1.0 scope, release and traceability baseline

- Status: Accepted
- Governing issue: #53
- Parent blueprint: #45
- Knowledge gate: #61 (Approved)
- Runtime implementation gate: #54
- Accepted: 2026-07-19

## Purpose

This document is the governed planning baseline for UC Rust 1.0. It makes release ownership, proof journeys, profile support, offline behavior, authority, quality evidence and economic evidence explicit. A capability not listed as in scope is not silently required for 1.0.

## Scope rules

1. UC-BoK is normative for Unified Commerce concepts. UC Rust owns implementation, runtime and evidence decisions.
2. A capability enters 1.0 only when it has an owner, target release, proof journey and measurable exit evidence.
3. `planned` does not mean `committed to 1.0`; only rows marked `in-scope` are release commitments.
4. Stable UC-BoK identifiers must replace provisional mappings before RC. Missing identifiers are visible gaps, not invented authority.
5. Central and store-edge profiles share canonical Operations. A profile may expose only a governed subset.
6. Offline support is declared per capability; it is never inferred from edge deployment.
7. Provider-specific payment, fiscal, identity and authorization behavior remains behind governed ports.

## Offline classes

| Class | Meaning |
|---|---|
| O0 | Online only; operation must fail safely when its required authority is unavailable. |
| O1 | Read from a governed local snapshot with explicit freshness limit. |
| O2 | Local mutation is durable and later synchronized; conflicts and idempotency are explicit. |
| O3 | Full supported journey continues during WAN partition and converges with no silent loss or duplication. |

## 1.0 capability matrix

| Capability / responsibility | UC Rust record | UC-BoK mapping | Owner | Release | Profiles | Offline | Data authority | Security / compatibility | Performance / economic evidence | Proof journey | Scope |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Canonical Operation invocation | RRR-0001 | provisional: Operation/Application Service | runtime-architecture | M1 | central, store-edge | O0/O2 by operation | operation-specific | typed context; deny-by-default hook; stable contract fixtures | QAR-0001/0002; ER-0001 | invoke one unchanged Operation through two adapters | in-scope |
| Basket | CR-0001 | provisional: Basket | commerce-domain | M2 | central, store-edge | O2 | store-edge while disconnected; central after convergence | tenant and actor checks; schema/version compatibility | invocation latency, memory and cost per basket mutation | create, modify and persist basket through two adapters and two stores | in-scope |
| Pricing and promotion calculation | CR-0002 | provisional: Pricing / Promotion | commercial-domain | M3 | central, store-edge | O1 | commercial authority publishes governed snapshots | signed/versioned inputs; deterministic fixtures | QAR-0001; calculation CPU/allocation; cost per calculation | reproduce commercial result and explanation from canonical fixture | in-scope |
| Product/catalog reference | planned CR | provisional: Product / Assortment | product-domain | M3 | central, store-edge | O1 | central product authority | snapshot compatibility and tenant/location scope | snapshot size, load time, freshness cost | basket resolves referenced sellable items while offline | in-scope |
| Checkout | planned CR | provisional: Checkout | checkout-domain | M4 | central, store-edge | O3 for supported journey | local checkout state until durable convergence | authorization, idempotency, audit and state-machine compatibility | latency and cost per completed checkout | commercial basket becomes durable checkout outcome | in-scope |
| Payment provider boundary | planned RRR | provisional: Payment | payments-integration | M4 | central, store-edge | O0 or provider-declared bounded offline mode | external PSP/acquirer | no generic offline promise; evidence per provider and version | provider latency/failure/cost contribution | authorized result is recorded without duplicating effects | in-scope boundary; provider implementations integrated |
| Fiscal provider boundary | planned RRR | provisional: Fiscalization | fiscal-integration | M4 | central, store-edge | provider/country-specific | external fiscal authority/provider | country/version compatibility and audit evidence | provider latency/failure/cost contribution | fiscal evidence is linked to checkout/order without embedding provider logic | in-scope boundary; country implementations deferred |
| Order creation | planned CR | provisional: Order | order-domain | M4 | central, store-edge | O2/O3 | store-edge during partition; central after convergence | immutable identity, idempotent creation, tenant isolation | cost per order; publication latency | completed checkout creates exactly one durable order | in-scope |
| Durable publication / outbox | planned RRR | provisional: Event Publication | reliability | M4 | central, store-edge | O2 | local durable store until acknowledged | schema compatibility and replay safety | backlog, publish latency and cost per effect | restart/retry cannot lose or duplicate business effects | in-scope |
| Edge synchronization and convergence | planned RRR | provisional: Synchronization | distributed-systems | M5 | store-edge, central | O3 | declared per dataset/operation | compatibility envelope; no silent conflict resolution | convergence time, bandwidth and cost per synchronized unit | partition, duplicate, reorder, restart and resync harness | in-scope |
| Capability and data-authority manifests | planned AR/RRR | provisional: Capability Management | architecture | M5 | central, store-edge | O1/O2 | governance/control plane | signed and versioned compatibility envelope | manifest size/load and fleet cost | runtime rejects unsupported profile/capability combinations | in-scope |
| Fleet identity, desired/actual state and safe rollout | planned CR/RRR | provisional: Store Systems Management | fleet-platform | M6 | control plane, store-edge | O1 | fleet control plane; edge owns actual local state | signed artifacts, canary, pause, rollback, drift evidence | rollout bandwidth/time and cost per managed edge | interrupted rollout preserves service and data | in-scope |
| Authentication boundary | planned RRR | provisional: Identity and Access | security | M7 | central, store-edge | O0/O1 subject to token policy | external IdP; runtime validates evidence | OIDC adapter, key rotation, expiry and tenant evidence | validation latency and operating cost | canonical operation receives verified identity context | in-scope boundary |
| Authorization boundary | planned RRR | provisional: Authorization | security | M7 | central, store-edge | O0/O1 under explicit cached-policy rules | authorization service/model | Zanzibar-compatible adapter; deny by default; model compatibility | decision latency/cache cost | cross-tenant denial and permitted operation evidence | in-scope boundary |
| Supply-chain, signing and provenance | planned QAR/RRR | provisional: Platform Security | security/release | M7 | build, control plane, edge | not applicable | release governance | SBOM, signatures, provenance and verification | build/release cost and verification latency | untrusted artifact cannot enter managed rollout | in-scope |
| Operational readiness and recovery | planned QAR/RRR | provisional: Service Management | reliability | M8 | all | profile-specific | service owner | SLI/SLO, backup/restore, incident bundle and rollback | accepted latency, allocation, memory, RPO/RTO and cost budgets | load, soak, restore and failure exercises | in-scope |
| Economics by Design reporting | ER-0001 plus planned ERs | EbD authority | economics | M1–M8 | all | not applicable | economic model owner | correlation IDs must not expose sensitive payloads | cost per invocation, basket mutation, calculation, order, store/day and tenant/month | reproduce cost report from accepted technical evidence | in-scope |

## Explicitly deferred beyond 1.0

The following are not hidden prerequisites for 1.0:

- full OMS and store-fulfilment suite;
- loyalty, customer profile and clienteling;
- inventory optimization and replenishment;
- native mobile/superapp experiences;
- smart checkout, RFID, computer vision and IoT implementations;
- embedded or agentic AI business decisions;
- arbitrary dynamic plugin loading or stable third-party ABI;
- warehouse-edge production profile;
- country-complete fiscal implementations;
- universal offline payment;
- complete analytics/BI product;
- custom consensus, broker or async executor.

They may integrate later through the same Operation, authority and extension rules. Entering the roadmap requires a named owner, UC-BoK disposition, quality/economic budget and accepted gate change.

## Release train and gates

| Release | Accountable outcome | Entry dependency | Exit evidence | Blocks |
|---|---|---|---|---|
| M0 | Project and architecture baseline | charter, target architecture, quality model | accepted blueprint and gate #54 planning evidence | M1 implementation |
| M0.5 | Architecture Knowledge Foundation | #55 | gate #61 approved | normative record governance |
| M1 | Operation First Runtime Foundation | #54 approved | accepted ADR/RFC, multi-adapter proof, central/edge composition, runtime budgets | M2–M8 runtime consumers |
| M2 | Persistent Basket | M1 | dual persistence and adapter evidence; authorization; cost report | M3, M4 |
| M3 | Commercial Basket | M2 | deterministic pricing and governed offline inputs | M4 |
| M4 | Checkout and Order | M2, M3 | exactly-once business outcome evidence, outbox and audit | M5 full journey |
| M5 | Distributed Store Edge | M4 | partition/restart/reorder/resync proof and compatibility matrix | M6 fleet rollout |
| M6 | Fleet Control and Safe Delivery | M5 | signed staged rollout, pause/rollback and drift proof | RC |
| M7 | Identity, Security and Data Governance | M1–M6 touchpoints defined | isolation, deny-by-default, supply-chain and threat evidence | RC |
| M8 | Operational and Economic Readiness | M1–M7 evidence sources | SLO, recovery, performance, capacity and unit economics | RC |
| RC | Contract freeze and full rehearsal | M1–M8 exits accepted | compatibility, installation, upgrade, rollback, recovery and drift report | 1.0 |
| 1.0 | Reference release | RC accepted | signed release and accepted evidence bundle | post-1.0 roadmap |

## Critical path

```text
M0.5 (#61)
  -> planning baseline (#53)
  -> Operation contract (#47)
  -> lifecycle/composition (#48)
  -> observability/economics (#49)
  -> work model (#50)
  -> adapters (#51)
  -> extension boundaries (#52)
  -> runtime gate (#54)
  -> M1
  -> M2 Basket
  -> M3 Commercial Basket
  -> M4 Checkout/Order
  -> M5 Edge convergence
  -> M6 Fleet delivery
  -> RC
  -> 1.0

M7 Security and M8 Operational/Economic Readiness run across the delivery train and must converge before RC.
```

## Compatibility and migration expectations

- Public Operation identifiers, typed contracts and evidence fixtures are versioned and migration-aware.
- Backward compatibility is required within an accepted compatibility window; breaking changes require an accepted decision and migration plan.
- Central, edge, manifest and adapter versions must be checked against an explicit compatibility envelope.
- Persistent state changes require upgrade, rollback and interrupted-migration proof.
- Events and synchronization messages require replay-safe versioning; consumers must not infer compatibility from semantic similarity.
- Existing ADR/RFC/session artifacts follow the approved migration policy and registry.
- Before RC, provisional UC-BoK mappings must become stable identifiers, be explicitly marked intentionally divergent, or receive a time-bounded waiver.

## Change control

A material change is any addition, removal or modification that affects the proving journey, release dependency, public contract, profile/offline promise, data authority, security guarantee, compatibility window, performance budget or economic unit.

A material change requires:

1. an issue identifying affected capability rows and releases;
2. architectural disposition through ADR/RFC when contracts or boundaries change;
3. UC-BoK and EbD impact disposition;
4. updated dependency graph, scope matrix and measurable evidence;
5. named owner and gate approver;
6. explicit acceptance. Silence is not approval.

Emergency or exploratory work may use a bounded waiver, but the waiver must state owner, scope, rationale, expiry and required reconciliation evidence.

## Acceptance basis

This baseline is Accepted because its governing dependencies have attributable accepted dispositions:

- #11 project charter accepted;
- #12 target architecture accepted and aligned with the Operation/runtime/realization model;
- #38 UC-BoK reference implementation and traceability contract accepted through ADR-0030;
- #41 market evidence dispositions accepted through ADR-0031;
- #68 Governed Capability Realization accepted through ADR-0024.

Acceptance approves the governed 1.0 planning baseline. It does not claim that M1–M8 executable evidence already exists. Those proofs remain mandatory at their respective release gates.