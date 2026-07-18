# Scheduled, Background and Worker Execution Test Plan

- Governing issue: #50
- Governing decision: ADR-0027

## Objective

Prove that schedules, queues, event consumers and background processors invoke canonical Operations without private business procedures, unsafe duplicate effects, unbounded retries or hidden central/edge authority.

## Canonical invocation

- Invoke the same Operation through an interactive adapter and a worker adapter.
- Verify identical typed input, authorization, idempotency and outcome semantics.
- Verify worker modules do not import repositories, provider SDKs or business-event publishers.
- Replace the trigger adapter without changing Operation contracts or fixtures.

## Schedules

- Ephemeral schedules disappear on restart and are allowed only for loss-tolerant work.
- Persistent schedules survive restart and materialize occurrences idempotently.
- Revisions affect future materialization only.
- Test time zones, daylight-saving gaps/overlaps and clock-skew bounds.
- Test `skip`, `run_once`, `bounded_catch_up` and `reconcile` misfire policies.
- Verify catch-up cannot create an unbounded burst.

## Ownership and leases

- One eligible executor acquires a fenced lease.
- A stale fencing token cannot commit checkpoint or disposition.
- Test renewal, expiry, crash and ownership transfer.
- Lease loss does not imply that prior effects did not occur.
- Leader-election loss affects coordination only, not business authority.

## Duplicate delivery and idempotency

- Deliver the same trigger repeatedly and verify one stable work identity.
- Start concurrent duplicates and verify idempotency convergence.
- Verify acknowledgement is not treated as exactly-once business proof.
- Verify non-idempotent or indeterminate outcomes enter reconciliation instead of automatic repetition.

## Retry and quarantine

- Retry only canonical retryable failures.
- Enforce maximum attempts, total age, attempt timeout, backoff and jitter.
- Do not blindly retry business rejection, cancellation or indeterminate outcomes.
- Quarantine invalid or exhausted work with a protected Failure Bundle.
- Release/replay requires authorization, reason and compatible target contract.
- Replay preserves causation and immutable attempt history.

## Concurrency and backpressure

- Enforce global, Operation, tenant/location and provider bulkhead limits.
- Preserve ordering only within declared partitions.
- Verify fairness prevents tenant/store starvation.
- Fill bounded queues and verify reject, defer, pause or shed behavior.
- Verify queue depth, oldest age, retry amplification and shed work are observable.
- Verify no unbounded memory growth during dependency outage.

## Central and edge placement

- `central_only` work is rejected at edge.
- `edge_only` work remains operable during WAN loss when local prerequisites are valid.
- `either_single_owner` cannot execute concurrently at central and edge.
- Handoff preserves identity, idempotency, correlation and disposition history.
- Reconnection does not silently duplicate locally completed work.

## Event consumers

- Accepted event revisions map deterministically to Work Envelopes.
- Unsupported revisions are rejected or quarantined.
- Source acknowledgement follows successful completion, durable retry/defer or quarantine.
- Restart and replay do not create new business semantics.

## Lifecycle

- Startup validates schemas, schedules and ownership providers before readiness.
- Quiesce stops intake/materialization according to work class.
- Drain completes bounded attempts and checkpoints eligible traversal.
- Shutdown relinquishes leases safely.
- Forced termination records incomplete attempts.
- Restart reconciles incomplete/indeterminate attempts before duplicate execution.

## Operator controls

- Pause, resume, cancel, retry, replay, reprioritize and quarantine release require authorization.
- Record actor, reason, time and state transition.
- Completed business outcomes cannot be rewritten.
- Controls cannot bypass idempotency, authority or compatibility rules.

## Observability and economics

- Reconstruct trigger -> Work Envelope -> attempt -> Operation -> realization -> disposition.
- Verify health distinguishes executor liveness from schedule obligation and queue-age health.
- Attribute wait time, execution, retries, duplicate suppression, coordination, storage, network and provider usage.
- Produce workload performance and cost-to-serve scorecards.

## Architecture enforcement

Automated checks prevent worker repository/provider imports, private worker business interfaces, trigger payload leakage, direct provider selection, direct business-event publication, unbounded retry/queue configuration and replay bypass.

## M1 evidence bundle

The bundle includes fixtures, schema validation, dependency checks, schedule/lease/retry output, central-edge placement proof, lifecycle recovery evidence, operator audit evidence and performance/cost scorecards.