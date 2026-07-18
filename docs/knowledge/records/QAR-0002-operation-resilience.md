---
id: uc-rust:QAR-0002
type: QAR
schema_version: 1
content_version: 0.1.0
title: Operation Invocation Resilience
summary: Resilience requirements for deterministic operation outcomes under dependency, timeout, retry and recovery failures.
status: Proposed
owners:
  - role: reliability-architecture
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Failure containment, retry safety, recovery and outcome classification for governed operation invocation.
non_goals:
  - Claiming exactly-once delivery
  - Hiding business rejection as infrastructure failure
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/60
relations:
  - type: constrains
    target: uc-rust:RRR-0001
    scope: failure classification, retry, deduplication and recovery
  - type: constrains
    target: uc-rust:CR-0001
    scope: basket mutation safety under retry and indeterminate outcomes
  - type: constrains
    target: uc-rust:CR-0002
    scope: pricing recalculation safety and explicit fallback behavior
review:
  required_roles: [reliability, runtime-architecture, security]
  reviewers: []
  disposition: pending
  review_class: continuous
  event_triggers: [incident, quality_budget_failure, dependency_or_provider_change, runtime_profile_change]
---

# QAR-0002 — Operation Invocation Resilience

## Quality scenario

When an invocation encounters a dependency outage, process interruption, deadline expiry, duplicate delivery or uncertain completion, the runtime contains the failure and returns or preserves a disposition that does not falsely claim success or definite failure.

The scenario covers synchronous, deferred and recovered execution. It distinguishes business rejection, transient infrastructure failure, permanent failure and indeterminate outcome.

## Measure

Measure at least:

- duplicate-effect rate under repeated invocation identity;
- percentage of failures classified with stable error semantics;
- percentage of indeterminate outcomes surfaced as indeterminate;
- recovery success and time after process interruption;
- deadline propagation and cancellation compliance;
- bounded retry count and retry amplification;
- evidence completeness for causation, identity and terminal disposition.

Acceptance requires zero known duplicate finalized effects in the canonical retry fixtures. No implementation may claim exactly-once delivery; effectively-once effects require durable identity, idempotency and business invariants.

## Environment and stimulus

Evidence must inject dependency unavailability, delayed responses, lost acknowledgements, process termination, replay and concurrent duplicates against declared central and edge profiles.

## Response

The runtime rejects unsafe retries, deduplicates where the contract permits, preserves causation and invocation identity, and exposes a stable terminal or indeterminate classification with recovery guidance.

## Evidence plan

Provide deterministic fault-injection tests, process-restart recovery fixtures, duplicate and replay tests, deadline/cancellation tests and an incident-style trace showing how an uncertain effect is reconciled.
