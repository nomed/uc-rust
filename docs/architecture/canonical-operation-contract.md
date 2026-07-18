# Canonical Operation and Invocation Contract

- Status: Design baseline
- Governing decision: ADR-0021
- Governing issue: #47
- Related: ADR-0024, RFC-0002, #48, #49, #50, #51

## Purpose

This specification defines the single application contract used by REST, gRPC, CLI, workers, schedulers, synchronization handlers and future delivery adapters to invoke Unified Commerce behavior.

A delivery mechanism may change. A Capability Realization may be native, delegated, composed, pipeline or hybrid. The canonical Operation contract does not change for either reason.

## Core model

```text
Capability
  contains Operations

Operation
  owns one canonical business use-case contract

Invocation
  executes an Operation in an explicit governed context

Capability Realization
  fulfills capability ports used by the Operation
```

## Operation identity

Every Operation has:

- a globally stable `operation_id`;
- a semantic contract version;
- an accountable capability and owner;
- typed command/input, outcome/output and error contracts;
- invocation mode and completion semantics;
- authorization, idempotency and transaction policies;
- effect-publication policy;
- supported runtime profiles;
- quality and Economics by Design metadata;
- canonical fixtures and conformance evidence.

An Operation identifier names business meaning, not transport, provider or implementation. Examples:

```text
uc-rust:basket.create
uc-rust:basket.add-item
uc-rust:pricing.calculate-commercial-outcome
```

Forbidden examples:

```text
POST /basket
CallRGK
KafkaCreateOrderHandler
```

Breaking semantic changes require a new major contract version. Transport revisions and provider adapter revisions do not change the Operation version unless canonical meaning changes.

## Canonical Rust shape

The implementation may refine syntax, but the semantic boundary is:

```rust
pub trait Operation: Send + Sync + 'static {
    type Input: OperationInput;
    type Output: OperationOutput;
    type Error: OperationError;

    const DESCRIPTOR: &'static OperationDescriptor;

    fn invoke<'a>(
        &'a self,
        context: &'a InvocationContext,
        input: Self::Input,
    ) -> OperationFuture<'a, Result<InvocationOutcome<Self::Output>, Self::Error>>;
}
```

The canonical contract is async-capable because delivery adapters, persistence, policy and delegated realizations may require asynchronous work. A purely local Operation may complete immediately without creating a second synchronous business interface.

`OperationFuture` may be implemented using stable language/runtime facilities or an accepted dependency. UC Rust does not build a custom executor.

## Static and dynamic boundaries

- Concrete Operations use static typing for input, output and error contracts.
- Runtime registration and delivery dispatch may use type-erased descriptors and invocation adapters at the outer composition boundary.
- Type erasure must not convert canonical contracts into untyped maps or provider payloads.
- Application code receives typed capability ports, not an unrestricted registry or service locator.
- Dynamic binary loading is not required for 1.0.

This preserves compile-time correctness inside Operations while allowing runtime discovery and adapter-neutral dispatch.

## Invocation context

`InvocationContext` is immutable from the Operation perspective and contains only governed execution facts:

- invocation identifier;
- correlation and causation identifiers;
- tenant and legal-entity scope;
- verified actor/subject identity and authentication evidence reference;
- channel, touchpoint, store and device scope where applicable;
- runtime profile and node identity;
- locale, currency and jurisdiction context when declared by the Operation;
- absolute deadline and cancellation signal;
- idempotency key and scope when required;
- requested Operation semantic version;
- trace and economic correlation handles;
- policy/configuration revision references;
- data-classification and consent evidence references where required.

The context does not contain repositories, provider clients, mutable global state, arbitrary headers or transport-specific request objects.

Adapters map approved transport metadata into canonical context fields. Unrecognized metadata remains outside the application boundary unless an accepted contract explicitly introduces it.

## Invocation modes

Every Operation declares one mode:

| Mode | Meaning |
|---|---|
| `request_response` | A final or explicitly provisional outcome is returned during the invocation. |
| `accepted_async` | Durable acceptance is returned; completion is observed through a canonical status/event contract. |
| `long_running` | A durable operation handle represents governed progress, cancellation and completion. |
| `streaming` | Deferred beyond 1.0 unless a capability has an accepted semantic need; transport streaming alone does not justify it. |

Workers and schedulers invoke the same Operation contract. They do not gain a private `run()` business interface.

## Deadlines and cancellation

- The invocation has one absolute deadline propagated from the adapter or assigned by accepted policy.
- Child capability calls receive a deadline no later than the parent deadline.
- Cancellation is cooperative and never presented as proof that an external effect did not occur.
- An Operation checks cancellation before beginning new irreversible work and at declared safe points.
- Once an outcome can be indeterminate, cancellation returns the canonical indeterminate disposition and reconciliation reference rather than a false failure claim.

## Authorization

Authorization is deny-by-default and occurs before protected business behavior. An Operation descriptor declares:

- required action/permission identifiers;
- resource derivation rules;
- whether a precondition or post-state authorization check is required;
- whether delegated realizations require downstream authorization evidence;
- audit obligations.

Delivery adapters may authenticate and perform coarse admission checks, but they cannot replace application-level authorization.

## Idempotency

Each mutating Operation declares one policy:

- `not_applicable`;
- `required`;
- `optional_supported`;
- `provider_scoped` for a governed external authority.

The declaration includes:

- key scope: tenant, actor, resource and Operation version;
- retention period;
- equivalence rule for repeated inputs;
- stored disposition and outcome replay behavior;
- handling of concurrent duplicates;
- reconciliation behavior for indeterminate external execution.

A retryable transport error does not imply that repeating the business Operation is safe.

## Transaction and unit-of-work ownership

The Operation owns the application transaction policy; repositories and adapters do not decide it implicitly.

Supported policies:

| Policy | Meaning |
|---|---|
| `none` | Pure/read-only behavior or external authority with no local atomic mutation. |
| `read_snapshot` | Governed consistent read scope. |
| `local_atomic` | One local unit of work owns state mutation and durable effects. |
| `saga` | Multiple authorities require explicit steps, compensation and reconciliation. |
| `external_authority` | An external system owns the authoritative transition; UC Rust records request, disposition and evidence explicitly. |

For `local_atomic`, state mutation, idempotency disposition and outbox/effect records commit atomically where required. Network calls are not held inside a local database transaction unless an accepted provider-specific decision proves the safety and cost.

## Effect publication

An Operation returns an `InvocationOutcome` containing the canonical output plus effect dispositions. Effects are not arbitrary callbacks.

Effect categories include:

- durable domain/integration events;
- outbox messages;
- audit evidence;
- reconciliation work;
- deferred follow-up Operations;
- protected operational diagnostics.

The Operation declares which effects must be atomic with local state and which may occur after commit. Delivery adapters never publish business effects independently.

## Outcome states

Every invocation ends in one canonical disposition:

- `succeeded`;
- `rejected` — valid request refused by business/policy semantics;
- `failed` — execution failed and no authoritative success is known;
- `accepted` — durable asynchronous acceptance;
- `indeterminate` — an external or interrupted transition may have occurred and requires reconciliation;
- `cancelled` — cancellation completed at a declared safe point;
- `timed_out` — deadline reached with explicit certainty/indeterminacy metadata.

## Error taxonomy

Errors are stable semantic dispositions, not raw exceptions or transport statuses.

| Class | Typical meaning | Retry rule |
|---|---|---|
| `invalid_input` | Canonical contract violation | Never without changed input |
| `not_authorized` | Actor lacks permission | Never without changed authority |
| `business_rejection` | Business invariant or policy rejection | Only after relevant state changes |
| `conflict` | Version/state conflict | Retry only with refreshed state/policy |
| `dependency_unavailable` | Required realization unavailable | Policy-controlled and deadline-bounded |
| `deadline_exceeded` | Invocation deadline elapsed | Depends on certainty and idempotency |
| `cancelled` | Safe cancellation completed | Caller decision |
| `indeterminate` | Effect may have occurred | Reconcile; do not blind-retry |
| `internal_failure` | Unexpected implementation failure | Bounded retry only when policy allows |
| `unsupported` | Profile/version/realization cannot satisfy contract | Rebind, reconfigure or reject |

Provider diagnostics are preserved in protected evidence but mapped to canonical errors at the provider boundary.

## Operation descriptor

Machine-readable Operation metadata is governed by `governance/schemas/operation-manifest.schema.json`. It supports:

- capability discovery;
- adapter registration;
- authorization policy binding;
- runtime-profile compatibility;
- Capability Binder eligibility;
- observability and economic attribution;
- fixture and conformance-kit discovery.

Descriptors are immutable within a released contract version and included in compatibility evidence.

## Observability and Economics by Design

Every invocation emits, through ports rather than business-logic dependencies:

- Operation ID and semantic version;
- invocation/correlation identifiers;
- runtime profile and selected realization identity;
- final disposition and canonical error class;
- deadline/cancellation/retry counters;
- duration, CPU/allocation and relevant resource measures;
- provider latency and attribution where applicable;
- economic unit and cost attribution references;
- effect and reconciliation counts.

Sensitive business payloads, identity attributes and provider diagnostics are not copied into telemetry by default.

## Adapter contract

A delivery adapter may:

1. decode transport data;
2. perform transport-shape validation;
3. authenticate and create verified evidence;
4. construct `InvocationContext`;
5. invoke the canonical Operation;
6. map canonical dispositions to transport responses;
7. propagate correlation and safe diagnostics.

It may not:

- contain business decisions;
- open repositories for business work;
- call capability providers directly;
- select a Capability Realization;
- own idempotency or transaction semantics;
- publish business events independently;
- invent transport-only business errors.

## Compatibility

Compatibility is evaluated across:

- Operation semantic version;
- input/output/error schema revisions;
- runtime profile;
- adapter revision;
- selected Capability Realization manifest;
- authorization/configuration policy revisions;
- persistent-state and event versions.

A compatible adapter can invoke the same Operation regardless of realization. An incompatible combination is rejected before business execution when feasible.

## 1.0 proving obligations

M1 must prove:

- one canonical Operation invoked through at least two adapter forms;
- static typed Operation implementation plus runtime descriptor/dispatch boundary;
- deadline and cancellation propagation;
- authorization and idempotency hooks;
- transaction/effect policy enforcement;
- canonical error mapping;
- Operation-level trace and economic correlation;
- architecture enforcement preventing adapter-to-repository/provider bypass.

The native/delegated semantic-equivalence proof is additionally governed by ADR-0024 and #68.