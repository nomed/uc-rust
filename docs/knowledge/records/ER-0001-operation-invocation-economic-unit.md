---
id: uc-rust:ER-0001
type: ER
schema_version: 1
content_version: 0.1.0
title: Operation Invocation Economic Unit
summary: Economic unit and measurement contract for the marginal cost of governed operation invocation.
status: Proposed
owners:
  - role: economics-by-design
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Cost attribution, measurement boundaries and economic thresholds for one completed or dispositioned operation invocation.
non_goals:
  - Product pricing or customer-facing commercial policy
  - Treating infrastructure spend as an undifferentiated monthly total
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/60
relations:
  - type: realizes_concept
    target: ebd:UNIT-OPERATION-INVOCATION
  - type: constrains
    target: uc-rust:RRR-0001
    scope: mandatory economic correlation and measurable runtime cost drivers
  - type: constrains
    target: uc-rust:CR-0002
    scope: pricing invocation cost visibility by runtime profile
review:
  required_roles: [economics-by-design, runtime-architecture, finance]
  reviewers: []
  disposition: pending
  review_class: release
  event_triggers: [economic_threshold_breach, dependency_or_provider_change, runtime_profile_change, release_gate]
---

# ER-0001 — Operation Invocation Economic Unit

## Economic scope

The governed economic unit is one operation invocation reaching a stable disposition: rejected, completed, failed, timed out or indeterminate. Measurements distinguish attempted, admitted, executed and completed units so retries, duplicates and rejected work are not hidden.

The unit applies across central and edge profiles while preserving profile-specific resource and provider costs.

## Economic constraints

1. Every measured invocation identifies operation, runtime profile, artifact revision, environment, disposition and correlation identity.
2. Cost allocation distinguishes runtime baseline, admission, application execution, persistence, external dependencies, telemetry, retry/recovery and retained evidence.
3. Duplicate and replay work are attributed rather than averaged away.
4. Idle profile cost is reported separately from marginal invocation cost.
5. Provider-specific pricing is an input to the calculation, not the identity of the economic unit.
6. Missing measurement is reported as unknown and never interpreted as zero cost.
7. Economic thresholds may block a release gate but do not mutate normative lifecycle automatically.

## Cost drivers

Minimum drivers are CPU time, allocated memory duration, durable reads/writes, network transfer, external provider calls, telemetry volume, retained idempotency/recovery metadata and retry amplification.

## Allocation and measurement

Measurements must declare observation interval, workload distribution, profile utilization, shared-cost allocation rule, currency, price-book revision and confidence. Central and store-edge results remain separate before any weighted portfolio aggregation.

## Initial gates

The Runtime Foundation review must establish:

- marginal cost per admitted and completed invocation by profile;
- idle cost per runtime profile;
- cost amplification under retry and failure scenarios;
- telemetry and retained-evidence cost per invocation;
- an explicit threshold or documented reason for deferral before 1.0 release disposition.

## Evidence plan

Produce a reproducible workload manifest, raw resource observations, price-book or cost-model revision, allocation calculation and result summary linked by immutable revision or digest. Compare nominal, failure/retry and offline-reconciliation workloads.
