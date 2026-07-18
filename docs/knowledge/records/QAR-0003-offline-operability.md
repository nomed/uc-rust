---
id: uc-rust:QAR-0003
type: QAR
schema_version: 1
content_version: 0.1.0
title: Offline Operability
summary: Quality requirements for safe bounded operation during loss of central connectivity and subsequent reconciliation.
status: Proposed
owners:
  - role: edge-architecture
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Availability, bounded autonomy, data and rule staleness, durability and reconciliation for offline-capable runtime profiles.
non_goals:
  - Requiring every capability to operate offline
  - Allowing offline mode to bypass legal, fiscal, security or ownership constraints
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/60
relations:
  - type: constrains
    target: uc-rust:CR-0001
    scope: basket mutation and finalization during central disconnection
  - type: constrains
    target: uc-rust:CR-0002
    scope: pricing with bounded local rule and data freshness
  - type: constrains
    target: uc-rust:RRR-0001
    scope: deferred invocation, durability and reconciliation semantics
review:
  required_roles: [edge-architecture, reliability, security, domain-architecture]
  reviewers: []
  disposition: pending
  review_class: continuous
  event_triggers: [incident, fiscal_or_legal_change, quality_budget_failure, runtime_profile_change]
---

# QAR-0003 — Offline Operability

## Quality scenario

When central connectivity becomes unavailable, an explicitly offline-capable runtime profile continues only the declared operations using locally available authorities, rules and data within approved freshness and risk bounds. When connectivity returns, durable local effects reconcile without silent loss, duplicate finalization or hidden conflict resolution.

## Measure

Measure at least:

- time to detect loss and restoration of connectivity;
- percentage of declared offline operations remaining available;
- local durable-write loss rate;
- reconciliation completion time and backlog age;
- duplicate-effect and unresolved-conflict rate;
- percentage of operations blocked correctly because required authority or freshness was unavailable;
- age of rule, identity, product, pricing and policy data at execution time;
- audit completeness across disconnection and recovery.

Acceptance requires zero silent data loss in canonical interruption fixtures and zero silent weakening of legal, fiscal or security controls.

## Environment and stimulus

Evidence must cover abrupt network partition, intermittent connectivity, process restart while disconnected, local storage pressure, stale rules, conflicting remote changes and restoration with backlog replay.

## Response

The runtime declares its operating mode, enforces the capability-specific offline contract, persists permitted effects, blocks unsafe operations, exposes degraded status and reconciles through explicit deterministic policies.

## Evidence plan

Provide central-to-edge partition tests, restart and disk-pressure fixtures, stale-rule and blocked-operation examples, conflict/reconciliation scenarios and an auditable trace from local invocation through central convergence.
