---
id: uc-rust:QAR-0001
type: QAR
schema_version: 1
content_version: 0.1.0
title: Operation Invocation Latency
summary: Measurable latency requirements for governed operation invocation across central and edge runtime profiles.
status: Proposed
owners:
  - role: performance-architecture
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Runtime overhead and end-to-end latency budgets for synchronous operation invocation under declared workloads.
non_goals:
  - Universal latency target independent of workload and environment
  - Business processing time hidden inside runtime overhead
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/60
relations:
  - type: constrains
    target: uc-rust:RRR-0001
    scope: invocation pipeline latency and overhead
  - type: constrains
    target: uc-rust:CR-0002
    scope: synchronous pricing response profiles
review:
  required_roles: [performance-architecture, runtime-architecture, reliability]
  reviewers: []
  disposition: pending
  review_class: release
  event_triggers: [quality_budget_failure, runtime_profile_change, release_gate]
---

# QAR-0001 — Operation Invocation Latency

## Quality scenario

When an authenticated caller invokes a registered synchronous operation under the declared nominal workload, the runtime accepts, contextualizes, authorizes, dispatches and classifies the invocation without adding uncontrolled latency or obscuring business-processing time.

The scenario must declare runtime profile, hardware class, concurrency, payload class, dependency mode, warm/cold state and observation window. Central and store-edge measurements are reported separately.

## Measure

The required measures are:

- invocation-pipeline overhead at p50, p95 and p99;
- end-to-end operation latency at p50, p95 and p99;
- timeout and deadline-exceeded rate;
- queue or admission delay where present;
- percentage of observations with complete trace and economic correlation.

Initial acceptance budgets are placeholders to be dispositioned by the Runtime Foundation review:

- p95 runtime-only overhead must be explicitly budgeted per profile and may not be hidden inside application time;
- p99 end-to-end targets must be declared by the consuming CR or interface profile;
- every breached budget produces a quality finding rather than silently relaxing the threshold.

## Environment and stimulus

Evidence must identify commit or artifact digest, runtime profile, processor and memory class, concurrency, request distribution, dependency stubs or providers, telemetry configuration and benchmark duration.

## Response

The runtime returns a stable disposition within the caller deadline, emits correlated timing evidence and distinguishes admission, runtime overhead, application execution and dependency time.

## Evidence plan

Produce reproducible local and CI benchmark fixtures, at least one central and one edge profile result, deadline/cancellation tests, and a regression comparison against the accepted baseline. Mutable dashboards without captured revision and observation time are insufficient.
