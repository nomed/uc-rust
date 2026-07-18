# Edge capability and data authority matrix

- Status: Draft baseline
- Governing issue: #39
- Related issue: #40
- Last updated: 2026-07-18

## Purpose

This document is the authoritative template for deciding what can execute at the edge, which data is available, who owns each state transition, and how disconnected work converges.

No edge-capable operation may be implemented without completed rows for its capability and affected data.

## Capability matrix

| Capability / operation | UC-BoK ID | Central | Store edge | Offline class | Required local data | Maximum acceptable staleness | Local writes | Sync direction | Conflict strategy | High-risk restriction | Evidence status |
|---|---|---:|---:|---|---|---|---|---|---|---|---|
| Create basket | TBD | yes | yes | offline-capable | tenant, location, currency, configuration | configuration validity window | edge-authoritative basket | edge→central as required | single owner per basket | none initially | Planned |
| Add/update/remove basket line | TBD | yes | yes | offline-capable | product snapshot, pricing inputs | capability-specific | edge-authoritative basket | edge→central as required | optimistic version on owning runtime | reject invalid/stale rule set | Planned |
| Calculate commercial basket | TBD | yes | yes | offline-capable-with-limits | product, price, promotion snapshots | declared per snapshot | calculation trace/snapshot | edge→central evidence | deterministic recalculation or preserve accepted snapshot | unsupported promotions become unavailable, never silently ignored | Planned |
| Identify customer | TBD | yes | conditional | degraded-offline | customer lookup snapshot/token | privacy and risk policy | limited local association | both | central authority with explicit merge | sensitive changes online-required | Planned |
| Loyalty earn | TBD | yes | conditional | offline-capable-with-limits | membership and earning policy | declared validity | append-only earning event | edge→central | idempotent append and reconciliation | fraud threshold | Planned |
| Loyalty redeem | TBD | yes | conditional | online-required by default | balance/authorization evidence | near-current | reservation/redemption | both | central authority or allocated offline budget | deny when guarantee unavailable | Planned |
| Checkout/finalize sale | TBD | yes | yes | offline-capable-with-limits | complete basket, fiscal/payment capability | operation-specific | sale/document/outbox | edge→central | immutable locally committed transaction + reconciliation | payment/fiscal limitations | Planned |
| Payment authorization | TBD | external | provider-specific | online-required by default | PSP/provider state | provider-defined | external/provider evidence | external + edge→central | provider contract | no generic offline promise | Planned |
| Store inventory movement | TBD | yes | conditional | offline-capable-with-limits | local stock view | operation-specific | local movement event | edge→central | append-only movement; central global convergence | global availability not strongly guaranteed offline | Planned |
| Global availability/reservation | TBD | yes | read-only/none | online-required | central/global inventory | current | central only | central→edge view | central authority | deny or offer alternative when offline | Planned |
| Order lookup | TBD | yes | conditional | read-only-offline | order replica | policy-specific | none | central→edge | central authority | stale indicator required | Planned |
| Order creation/fulfillment | TBD | yes | conditional | degraded-offline | order/fulfillment policy and inventory | operation-specific | local intent/event | edge→central | semantic reconciliation | routing requiring global view online-required | Planned |
| Authorization check | TBD | yes | yes for approved subset | offline-capable-with-limits | identity evidence and policy/relationship snapshot | risk-specific | audit only | both | central relationship authority | deny when local evidence insufficient | Planned |
| Fleet update | n/a | control plane | edge agent | degraded-offline | signed manifest and staged artifact | manifest validity | installation state | both | desired/actual reconciliation | never update during unsafe retail state | Planned |

## Data authority matrix

| Data class | Category | Primary authority | Edge representation | Version/checkpoint | Freshness policy | Edge mutation | Convergence | Conflict handling | Loss tolerance |
|---|---|---|---|---|---|---|---|---|---|
| Product master | central-authoritative replica | central product source | versioned subset/snapshot | source revision + checkpoint | explicit validity | none or local overlay only if governed | central→edge snapshot/delta | central wins; overlays separately modelled | reconstructible, but outage must not remove active valid snapshot |
| Prices | central-authoritative replica | pricing authority | effective-dated snapshot | price-list revision | effective period and expiry | no direct mutation | central→edge | reject unsupported overlap/inconsistency | active accepted snapshot must remain available until policy expiry |
| Promotions | central-authoritative replica | promotion authority | executable supported subset | rule-set revision | effective period and dependency completeness | no | central→edge | unsupported rule cannot be silently skipped | valid subset required for promised offline capability |
| Configuration | central desired state | control plane | signed validated revision | schema + revision + checksum | validity/activation window | no arbitrary local edits | central→edge | desired/actual drift | previous known-good revision retained |
| Authorization relationships/policy | central authority | authorization system | filtered snapshot/materialized decision inputs | schema revision + causal token/checkpoint | risk-specific TTL | local audit only | central→edge; audit edge→central | deny if guarantee unavailable | no privilege expansion due to staleness |
| Basket | runtime-owner authoritative | runtime that owns basket | full local aggregate | aggregate version | immediate local | yes | optional/required edge→central | optimistic concurrency; explicit transfer of ownership | zero committed basket effect loss |
| Sale/transaction | edge-authoritative at creation | committing runtime | immutable local record | global ID + local sequence | n/a | append/commit | edge→central | idempotent append and reconciliation | zero loss, zero duplicate business effect |
| Fiscal document | edge/provider authoritative | fiscal component/runtime | immutable evidence | provider/document ID | n/a | append/commit | edge→central/archive | no overwrite; corrective document process | zero loss |
| Payment evidence | PSP/provider authoritative | PSP/acquirer | tokenized/reference evidence only | provider transaction ID | n/a | provider-controlled | provider + edge→central | provider reconciliation | core must not invent status |
| Outbox | edge-authoritative technical state | local runtime | durable queue | local monotonic sequence | n/a | append/ack | edge→central | retry/dedupe/dead-letter | zero loss before acknowledged durable acceptance |
| Customer snapshot | central-authoritative replica | CRM/customer domain | privacy-filtered subset | revision/checkpoint | explicit TTL | limited governed association | both where allowed | semantic merge/manual disposition | sensitive data minimization required |
| Inventory movement | event authority at source | committing location/system | local append-only movement | event ID/sequence | n/a | append | edge→central | dedupe and domain reconciliation | zero movement loss |
| Global inventory view | derived/converged view | inventory read model | cached/replicated view | model checkpoint | exposed staleness | no direct mutation | central→edge | recompute | may be stale but must be labelled |
| Telemetry | edge-originated operational evidence | edge + observability platform | buffered local events/metrics | timestamp + sequence | bounded retention | append | edge→central | dedupe/aggregation | controlled loss policy for noncritical metrics; critical audit durable |
| Fleet actual state | edge-reported | edge node | local signed inventory/state | report revision | periodic/event-driven | edge | edge→central | latest valid signed report, not wall-clock LWW alone | must survive temporary WAN outage |

## Mandatory completion rules

Before an operation becomes edge-capable:

1. all affected rows have stable identifiers and owners;
2. freshness and expiry behavior are executable;
3. writes identify an authority and durability boundary;
4. duplicate, delayed, reordered and replayed delivery is tested;
5. conflict behavior is semantic and tested;
6. unsupported/offline-restricted behavior is visible to clients;
7. authorization and privacy behavior is defined;
8. recovery, bootstrap and re-sync preserve local authoritative work;
9. performance and capacity budgets include prolonged offline backlog;
10. evidence is linked from the system quality model.
