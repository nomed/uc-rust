---
id: uc-rust:RRR-0001
type: RRR
schema_version: 1
content_version: 0.1.0
title: Operation Invocation
summary: Reusable runtime semantics for operation invocation.
status: Proposed
owners: [{role: runtime-architecture}]
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Runtime invocation semantics independent of capability and transport.
non_goals: [Business meaning of a specific operation, Mandatory protocol/framework/topology]
provenance:
  - {kind: issue, locator: https://github.com/nomed/uc-rust/issues/56}
relations:
  - {type: realizes_runtime_for, target: uc-rust:CR-0001}
review:
  required_roles: [runtime-architecture, reliability, security]
  reviewers: []
  disposition: pending
---

# RRR-0001 — Operation Invocation

## Responsibility
Accept an operation request, establish execution context, apply admission/idempotency rules, invoke an implementation and return a deterministic disposition with evidence.

## Boundary
Inputs: operation/version, invocation identity, actor/tenant, payload, causation/correlation, idempotency key, deadline, execution profile and trace context.

Outputs: accepted/rejected/completed/failed/timed-out/indeterminate disposition, result, stable error semantics, evidence and retry guidance.

## Lifecycle
Received; contextualized; admitted/rejected; deduplicated/registered; executing; terminal; evidence retained. Terminal states are immutable.

## Invariants
1. Invocation identity differs from business-operation identity.
2. Admission precedes side effects.
3. Deadlines are propagated and enforced where possible.
4. Acknowledgement does not imply durable completion unless declared.
5. Execution profile is explicit.
6. Exactly-once delivery is never claimed; effectively-once effects require durable identity, idempotency and business invariants.
7. Indeterminate outcomes are not reported as definite failure when an effect may have occurred.

## Failure model
Invalid request; authentication/authorization failure; unsupported version; admission/quota rejection; unavailable dependency; deadline exceeded; business rejection; implementation fault; concurrency conflict; indeterminate outcome.

## Profiles and budgets
Synchronous local/remote, asynchronous queued, edge/offline deferred, scheduled/agent-driven. Profiles declare admission overhead, in-flight memory, durable metadata, retention, concurrency, telemetry and recovery/replay cost; thresholds belong to QAR/ER.

## Observability and security
Record identities, causation, profile, admission, timestamps, terminal classification, retry/deduplication, trace, implementation and environment fingerprint. Validate authentication, record authorization authority, control replay, protect payload integrity, redact sensitive telemetry and constrain exhaustion.

## Acceptance evidence
Schema validation; duplicate/idempotency tests; deadline/cancellation tests; indeterminate-outcome handling; synchronous and deferred realizations; stable errors; trace/audit correlation; use by at least two operations or capabilities.

## ADR-0022 validation
Confirms Runtime Responsibility Record as a distinct family while Operation remains a concept in capability/interface models.
