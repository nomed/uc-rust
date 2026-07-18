# ADR-0027 — Scheduled, Background and Worker Execution Through Operations

- Status: Reviewable for acceptance
- Date: 2026-07-19
- Governing issue: #50
- Parent epic: #46
- Related: ADR-0021, ADR-0024, ADR-0025, ADR-0026, RFC-0002, #51, #52

## Context

UC Rust must execute scheduled work, queued work, event-triggered work and long-running background processing in central and edge profiles. These mechanisms must not create private business procedures, transport-specific services or broker-owned semantics that diverge from canonical Operations.

Distributed execution introduces duplicate delivery, lease loss, clock skew, retries, partial failure, poison inputs, backpressure and operator intervention. These concerns must be explicit without forcing a particular broker, scheduler or consensus technology.

## Decision

Every scheduled, queued, event-triggered or background unit of business work invokes a canonical Operation through the same governed invocation contract used by interactive delivery adapters.

```text
Trigger source
  -> trigger adapter
    -> Work Envelope
      -> admission and ownership
        -> canonical Operation invocation
          -> Operation outcome
            -> work disposition and evidence
```

Workers may own trigger decoding, lease handling, admission, checkpointing and disposition. They do not own business procedures, provider selection, repositories, transactions or independent business-event publication.

## Work model

The canonical work model separates:

- **Schedule Definition** — when and under which policy work should be materialized;
- **Work Envelope** — immutable identity, target Operation, canonical input reference, execution policy and correlation facts;
- **Work Attempt** — one bounded ownership period and invocation attempt;
- **Work Disposition** — completed, rejected, retry-scheduled, deferred, cancelled, dead-lettered, indeterminate or superseded;
- **Checkpoint** — durable progress only for explicitly checkpointable technical traversal; it cannot redefine Operation semantics.

A Work Envelope references a registered Operation ID and semantic version. It cannot name a private worker method or provider-specific endpoint.

## Ephemeral and persistent schedules

- `ephemeral` schedules exist only for the lifecycle of one runtime instance and are permitted only where missed execution is acceptable and explicitly declared;
- `persistent` schedules are durable, revisioned and recoverable, with explicit misfire, catch-up and duplicate-materialization policies;
- calendar/time-zone semantics, daylight-saving behavior and clock-skew tolerance are part of the schedule contract;
- changing a persistent schedule creates a new revision; in-flight materialized work retains the revision that created it.

A missed schedule never silently becomes a burst of catch-up work. The schedule declares `skip`, `run_once`, `bounded_catch_up` or `reconcile` behavior.

## Ownership, leases and coordination

Persistent work uses renewable, fenced leases or an equivalent ownership primitive. A lease contains work identity, owner instance, fencing token, acquisition time, expiry and revision.

- only the current fenced owner may commit attempt disposition or checkpoint state;
- lease expiry does not prove that the previous invocation had no effect;
- ownership transfer requires idempotency or reconciliation semantics appropriate to the Operation;
- leader election is permitted only for schedule materialization or partition coordination, never as business authority;
- the runtime does not implement distributed consensus internally; it consumes an approved coordination provider behind a typed port when required.

Single-instance and edge-local profiles may use local durable ownership without distributed coordination when deployment topology proves that only one eligible executor exists.

## Retries, idempotency and indeterminate outcomes

Trigger redelivery and Operation retry are different decisions.

- every mutating work item carries an idempotency scope compatible with ADR-0021;
- retry policy is bounded by attempt count, elapsed time, deadline, error class and reconciliation policy;
- retries use explicit backoff and jitter;
- business rejection is not retried unless the Operation contract explicitly permits a later re-evaluation;
- an indeterminate outcome enters reconciliation or operator review and is never automatically treated as a clean failure;
- poison input is quarantined with a protected Failure Bundle rather than retried indefinitely.

At-least-once trigger delivery is the default assumption. Exactly-once business effects are not claimed from broker acknowledgements; they require Operation-level idempotency, atomic state/effect recording or explicit reconciliation.

## Deadlines and cancellation

Each Work Envelope declares or derives:

- admission deadline;
- per-attempt execution deadline;
- maximum total age;
- cancellation behavior;
- shutdown checkpoint or relinquish behavior.

Cancellation is cooperative and follows ADR-0021. Cancelling a trigger does not prove that external or irreversible effects did not occur.

## Concurrency, fairness and backpressure

Concurrency is bounded by runtime profile, Operation class, tenant/legal scope, location, provider bulkhead and economic budget.

The runtime supports:

- global and per-Operation concurrency limits;
- partition keys for ordering-sensitive work;
- weighted fairness to prevent one tenant, store or workload from starving others;
- bounded queues and admission rejection/defer policies;
- provider-specific bulkheads without leaking provider identity into Operation contracts;
- load shedding only according to declared priority and legal/authority constraints.

Unbounded in-memory queues are forbidden. Backpressure must propagate to trigger intake or durable deferral and must be observable.

## Central and edge execution

Execution placement is governed by Operation profile compatibility, data/authority locality, offline class, realization eligibility, latency, resource envelope and cost-to-serve.

- `central_only` work cannot be opportunistically executed at edge;
- `edge_local` work remains operable without WAN when required inputs, manifests and authority are locally durable;
- `either` work requires a deterministic placement and ownership rule to avoid concurrent central/edge execution;
- handoff between edge and central preserves work identity, causation, idempotency and disposition history;
- clock or connectivity loss cannot create silent duplicate authority.

## Event consumers

An event consumer maps an accepted event contract to a Work Envelope. It does not contain business workflow logic.

Event acknowledgement occurs only after the declared durability point: successful Operation disposition, durable retry/defer record, or protected quarantine. Offset/ack commits cannot precede the durability guarantee they represent.

Event replay creates new attempts against the same or explicitly migrated semantic contract and is auditable. Replays cannot bypass current authorization, compatibility or safety rules.

## Operator controls

Governed controls include pause, resume, cancel, retry, replay, quarantine release, priority adjustment and bounded drain.

Each control:

- requires authorization and reason;
- records actor, timestamp, prior and new state;
- preserves immutable attempt history;
- cannot rewrite a completed business outcome;
- cannot bypass idempotency, authority, legal, fiscal or compatibility rules.

## Lifecycle integration

Workers participate in ADR-0025 lifecycle supervision.

- startup validates schedule/work schemas and ownership providers;
- quiesce stops new admission/materialization according to class;
- drain allows bounded completion;
- persistent workers checkpoint or relinquish leases safely;
- forced shutdown records incomplete attempts and recovery obligations;
- readiness reflects the mandatory work classes for the runtime profile, not merely thread/process liveness.

## Observability and economics

Every attempt emits ADR-0026 evidence linking trigger, schedule revision, Work Envelope, attempt, owner/lease, Operation, realization, disposition, retry/reconciliation state and economic attribution.

Required budgets include queue depth/age, lease churn, duplicate rate, retry amplification, CPU/memory, storage, network, provider usage and operator-intervention cost.

## Technology selection

No broker, distributed scheduler, leader-election system or workflow engine is mandatory. A technology is adopted only after measured need, failure-mode analysis, operational ownership, portability review and cost-to-serve scorecard.

## Rejected alternatives

- private `run()` business methods owned by workers;
- cron handlers that call repositories or providers directly;
- broker-specific payloads as canonical business contracts;
- claiming exactly-once effects from message delivery semantics alone;
- unbounded retry or in-memory queues;
- leader election as business authority;
- automatic retry of indeterminate outcomes;
- universal central execution that breaks edge offline operation;
- silent central/edge duplicate execution;
- operator replay that bypasses current governance.

## Design evidence

- `docs/architecture/scheduled-background-and-worker-execution.md`;
- `governance/schemas/work-envelope.schema.json`;
- `docs/testing/scheduled-background-worker-test-plan.md`;
- RFC-0002 integration.

## Implementation evidence required by M1

- the same Operation invoked interactively and through at least one worker/consumer adapter;
- persistent schedule materialization and misfire tests;
- lease fencing, loss and ownership-transfer tests;
- duplicate delivery and idempotency proof;
- bounded retry, poison input and indeterminate reconciliation proof;
- concurrency, fairness and backpressure tests;
- central/edge placement and WAN-loss proof;
- quiesce, drain, checkpoint and forced-shutdown recovery tests;
- operator-control authorization and audit evidence;
- workload performance and cost-to-serve scorecard.

This ADR can be accepted before executable evidence exists; gate #54 must not claim those proofs until produced.