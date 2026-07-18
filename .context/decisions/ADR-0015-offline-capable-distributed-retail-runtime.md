# ADR-0015 — Offline-capable distributed retail runtime

- Status: accepted
- Date: 2026-07-18
- Decision owners: Human project owner, Domain Architect
- Related issues: #31, #38, #39, nomed/uc-bok#10

## Context

Retail operations must continue when WAN connectivity is unavailable, degraded or intermittent. UC Rust therefore cannot be designed only as a centrally hosted request/response application. Selected capabilities must execute at peripheral locations while preserving one business model and converging safely with the central platform.

## Decision

UC Rust is a distributed Unified Commerce system with central and edge deployment profiles.

Central and edge profiles reuse the same domain model and canonical application operations. Edge-specific behavior is provided through composition, adapters, locally available capabilities and explicit operating policies; business procedures must not be reimplemented for the edge.

An edge runtime is an autonomous execution environment, not a passive cache. Each capability must declare exactly one supported offline classification:

- `offline-capable`;
- `offline-capable-with-limits`;
- `read-only-offline`;
- `degraded-offline`;
- `online-required`;
- `forbidden-offline`.

The architecture distinguishes:

1. cache: disposable, reconstructible data;
2. replica: versioned local copy of centrally authoritative data;
3. edge-authoritative data: durable data created locally that must survive disconnection and later converge.

Synchronization is a first-class platform capability. It owns durable checkpoints, snapshots and deltas, outbox/inbox processing, retry, deduplication, ordering, compatibility negotiation, reconciliation and observability.

Conflict resolution is defined per capability and data authority. Generic last-write-wins is forbidden unless an explicit accepted decision demonstrates that it preserves the domain semantics.

Central and edge API contracts retain the same semantics. Availability or guarantees may differ only through an explicit capability profile and runtime capability discovery.

## Required guarantees

- Edge-authoritative operations survive process restart and unbounded WAN outage.
- Reconnection never produces duplicate business effects.
- Delayed and out-of-order messages are handled explicitly.
- Bootstrap and full re-sync do not discard unsent local data.
- Central/edge compatibility is validated across application, DB schema, sync protocol, contracts, configuration, authorization model and UC-BoK revision.
- Offline authentication and authorization use an explicit risk, freshness and revocation policy.
- `offline` is a connectivity state and does not automatically mean operational failure.

## Consequences

- Capability and context mapping must include offline classifications, authority and consistency guarantees.
- Sync cannot be implemented as generic table replication without domain semantics.
- Contract evolution and migrations must account for mixed central/edge versions.
- Testing must include partitions, long disconnection, duplicate delivery, reorder, restart, corruption recovery and reconciliation.
- UC-BoK must evaluate the general retail principles through `nomed/uc-bok#10`.
