# Scheduled, Background and Worker Execution Through Operations

- Governing issue: #50
- Governing decision: ADR-0027

## Purpose

This specification defines the runtime contract by which schedules, queues, event consumers and background processors invoke canonical UC Rust Operations. It prevents work infrastructure from becoming a second application architecture.

## Core invariant

```text
all business work -> canonical Operation invocation
```

A trigger adapter may decode, validate trigger metadata, acquire ownership, construct an invocation context and record the technical disposition. It cannot implement business procedures, select Capability Realizations, access repositories directly or publish independent business effects.

## Canonical records

### Schedule Definition

A Schedule Definition contains:

- stable schedule ID and revision;
- target Operation ID and semantic version range;
- trigger type and time/calendar policy;
- runtime-profile and placement eligibility;
- input materialization policy;
- misfire/catch-up policy;
- concurrency, overlap and ordering policy;
- deadline, cancellation and maximum-age policy;
- owner and runbook references;
- economic and observability budget references.

### Work Envelope

A Work Envelope is immutable after admission and contains:

- work ID;
- source trigger identity and source revision;
- target Operation and semantic version;
- canonical input or durable input reference;
- tenant, legal entity, location and profile scope;
- correlation and causation IDs;
- priority and partition key;
- idempotency scope/key where required;
- admission and execution deadlines;
- retry, reconciliation and placement policies;
- created-at and maximum-age facts;
- configuration/binding revisions captured at admission where applicable.

### Work Attempt

An attempt records:

- attempt number and identity;
- executor/runtime instance;
- lease/fencing token;
- start/end timestamps;
- Operation invocation ID;
- canonical outcome;
- technical failure bundle reference;
- retry/defer/reconcile decision;
- checkpoint reference where allowed;
- economic and resource observations.

### Work Disposition

Allowed dispositions are:

- `completed`;
- `rejected`;
- `retry_scheduled`;
- `deferred`;
- `cancelled`;
- `dead_lettered`;
- `indeterminate`;
- `superseded`.

Disposition is append-only. Corrections create a new governed transition rather than rewriting attempt history.

## Schedule semantics

### Ephemeral

Ephemeral schedules are runtime-local and disappear with the process. They are valid only when loss or missed execution is accepted by contract. They cannot be used for fiscal, legal, reconciliation or otherwise durable obligations.

### Persistent

Persistent schedules are durable and revisioned. Materialization must be idempotent by `(schedule_id, revision, nominal_fire_time)` or an equivalent stable key.

Each schedule declares one misfire policy:

- `skip`;
- `run_once`;
- `bounded_catch_up`;
- `reconcile`.

Time zone, daylight-saving transitions, clock-skew tolerance and overlap behavior are explicit.

## Ownership and leases

Persistent work ownership uses fenced leases or a stronger equivalent.

A valid lease record includes:

```text
work_id
owner_instance_id
fencing_token
acquired_at
expires_at
lease_revision
```

Only the current fencing token may commit checkpoint or disposition. Lease renewal is bounded and observable. Lease loss causes cooperative cancellation where possible, but does not imply that prior effects did not occur.

Leader election may coordinate schedule materialization or partition assignment. It is not business authority and cannot replace Operation authorization or transaction policy.

## Delivery and idempotency

The runtime assumes at-least-once trigger delivery. Duplicate Work Envelopes or attempts must converge through:

1. stable work/materialization identity;
2. Operation-level idempotency;
3. atomic state/effect recording where local;
4. explicit reconciliation for external or indeterminate outcomes.

Broker acknowledgement, offset commit or queue deletion is a transport disposition, not proof of exactly-once business execution.

## Retry decision

Retry is computed from:

- canonical error class;
- retryability declared by the Operation;
- attempt and elapsed-time budgets;
- idempotency/reconciliation safety;
- current deadline and work age;
- dependency health and circuit state;
- operator policy.

Retry policy includes bounded attempts, exponential or policy-defined backoff and jitter. Business rejection, cancellation and indeterminate outcome are not blindly retried.

## Poison and dead-letter handling

Work is quarantined when input cannot be decoded/migrated, repeatedly violates a stable contract, exceeds retry budgets or requires human reconciliation.

Dead-letter/quarantine records preserve:

- original protected envelope or reference;
- schema/contract revisions;
- failure bundle;
- attempt history;
- owner/runbook;
- release/replay authorization policy.

## Concurrency and ordering

Concurrency policies may be declared globally and by Operation, tenant, legal entity, location, partition, realization/provider or priority class.

Ordering is guaranteed only within an explicitly named partition and only to the declared level. Global ordering is not inferred.

Fairness and starvation prevention are required for shared central runtimes. Edge profiles may use simpler local policies if resource and topology evidence justify them.

## Backpressure

Every queue or intake path is bounded. When capacity is reached, the configured behavior is one of:

- reject admission;
- durable defer;
- pause source consumption;
- shed explicitly non-mandatory work;
- degrade affected capability.

The runtime reports queue depth, oldest age, admission rejection, retry amplification and dropped/shed detail. Mandatory legal, security or reconciliation evidence cannot be silently dropped.

## Placement

Placement policy values are:

- `central_only`;
- `edge_only`;
- `prefer_central`;
- `prefer_edge`;
- `either_single_owner`.

Eligibility is resolved before economics using semantic compatibility, authority, security, data locality, offline promises, realization availability and resource budgets.

For `either_single_owner`, ownership is globally deterministic or explicitly handed off. Central and edge cannot concurrently claim the same work identity.

## Event-consumer contract

An event adapter:

1. validates the accepted event contract;
2. derives a stable source identity;
3. maps it to a canonical Work Envelope;
4. durably admits the work;
5. acknowledges the source only after the declared durability point.

Event-to-Operation mapping is declarative and versioned. Complex multi-step business behavior belongs in canonical Operations or an accepted governed process model, not in consumer callbacks.

## Operator controls

Supported controls are pause, resume, cancel, retry, replay, release quarantine, reprioritize and drain.

Controls are Operations or governed administrative commands with authorization, reason, audit evidence and immutable history. They cannot mutate historical outcomes or circumvent semantic/version compatibility.

## Lifecycle behavior

During quiesce:

- schedule materialization stops or is bounded by class;
- source intake pauses;
- new low-priority work is rejected/deferred.

During drain:

- admitted attempts complete within deadline;
- checkpointable traversal persists progress;
- leases are renewed only within shutdown budget;
- uncompleted work is relinquished with recovery evidence.

On next startup, incomplete attempts are reconciled before unsafe duplicate execution.

## Observability and health

Health is reported separately for:

- schedule materialization;
- queue/source intake;
- ownership/lease provider;
- executor pool;
- each mandatory work class;
- affected retail journey.

An executor process being live does not imply scheduled obligations are healthy. Queue-age and missed-schedule thresholds contribute to operational health.

## Economics by Design

Per-attempt observations include:

- wait and execution duration;
- CPU, allocation/memory and storage;
- source/broker and provider usage;
- retries and duplicate suppression;
- lease/coordination overhead;
- operator intervention;
- economic unit and allocation-model revision.

Infrastructure selection compares native timers/local durable queues, database-backed work, brokers and distributed schedulers only after semantic and reliability eligibility.

## Architecture enforcement

Automated checks must prevent:

- worker modules importing repositories or provider SDKs;
- worker-specific business service interfaces;
- source payload types leaking into Operation contracts;
- direct business event publication by trigger adapters;
- unbounded retry or queue configuration;
- acknowledgement before the required durability point;
- Operation bypass during replay or operator retry.