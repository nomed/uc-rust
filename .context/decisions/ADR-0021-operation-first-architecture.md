# ADR-0021 — Operation First Architecture

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #47
- Parent epic: #46
- Related: ADR-0002, ADR-0015, ADR-0018, ADR-0024, RFC-0002

## Context

UC Rust exposes the same Unified Commerce capabilities through REST, gRPC, workers, jobs, CLI, synchronization and future touchpoints. Allowing each adapter to own procedures would duplicate business logic, create semantic drift and make central/edge behavior diverge.

The canonical Operation contract must also remain stable when its underlying capability is realized natively, delegated to an external specialist system, composed from multiple realizations or executed as a governed pipeline.

## Decision

Every business use case is exposed as one canonical application **Operation**.

An Operation:

- has a stable identifier, semantic version, capability and accountable owner;
- declares typed input, output and stable canonical errors;
- executes through one async-capable shared invocation contract;
- receives an explicit immutable execution context;
- declares authorization, deadline, cancellation and idempotency semantics;
- owns the application transaction/unit-of-work and effect-publication policy;
- is independent from transport, scheduler, broker, provider and persistence technology;
- is reusable by central and edge runtime profiles;
- emits trace, metric and economic correlation data through ports;
- has canonical human-readable fixtures and contract tests;
- is invoked identically regardless of its selected Capability Realization.

REST controllers, gRPC services, CLI commands, workers, schedulers and synchronization handlers are delivery adapters. They decode, validate transport concerns, construct a governed invocation context, invoke an Operation and encode its canonical disposition. They do not implement business procedures and do not select or invoke external providers directly.

## Invocation contract

The normative design is `docs/architecture/canonical-operation-contract.md`.

The contract is async-capable because persistence, policy and delegated realizations may require asynchronous execution. Local Operations may complete immediately without introducing a second synchronous business API.

Concrete Operation implementations remain statically typed. Type erasure is permitted only at runtime registration and delivery dispatch boundaries and must not turn canonical contracts into arbitrary maps or provider payloads.

## Execution context

The invocation context contains governed execution facts such as invocation/correlation identity, tenant and legal entity, verified actor evidence, location/touchpoint scope, runtime profile, deadline, cancellation, idempotency scope, policy/configuration revisions and observability/economic handles.

It never contains repositories, provider clients, mutable global state, arbitrary transport headers or framework request objects.

## Authorization

Authorization is deny-by-default and is an application-level obligation declared by the Operation. Delivery adapters may authenticate and perform coarse admission checks, but cannot replace Operation authorization or resource derivation.

## Idempotency

Mutating Operations explicitly declare whether idempotency is required, optional, provider-scoped or not applicable. The policy owns key scope, retention, input-equivalence, concurrent duplicates, replay and indeterminate-outcome reconciliation.

A retryable transport error never implies that repeating the business Operation is safe.

## Transactions and effects

The Operation declares one transaction policy: `none`, `read_snapshot`, `local_atomic`, `saga` or `external_authority`.

For local atomic work, state changes, idempotency disposition and required outbox/effect records commit together. Multi-authority work uses explicit saga, compensation or reconciliation semantics rather than pretending to be one transaction.

Business effects are produced by the Operation outcome. Delivery adapters do not publish independent business events.

## Error and completion semantics

Canonical invocation dispositions include succeeded, rejected, failed, accepted, indeterminate, cancelled and timed out.

Stable error classes include invalid input, authorization failure, business rejection, conflict, dependency unavailable, deadline exceeded, cancellation, indeterminate outcome, unsupported combination and internal failure. Raw provider, transport and persistence errors do not cross the canonical boundary.

## Runtime relationship

`uc-runtime` composes and invokes Operations. It owns lifecycle and cross-cutting execution concerns, but does not absorb domain logic.

When an Operation depends on a capability, the runtime resolves an approved realization according to ADR-0024. The application contract remains canonical while the realization may be native, delegated, composed, pipeline or hybrid.

```text
Delivery adapter
  -> canonical Operation invocation
    -> Application Operation
      -> capability-oriented port
        -> Capability Binder
          -> governed Capability Realization
```

## Machine-readable descriptor

Operation metadata follows `governance/schemas/operation-manifest.schema.json`. Descriptors govern discovery, adapter registration, authorization binding, profile compatibility, Capability Binder eligibility, observability, Economics by Design and evidence discovery.

`governance/examples/create-basket.operation.json` is the initial canonical example.

## Architecture enforcement

`docs/architecture/operation-architecture-enforcement.md` defines the executable dependency rules for M1. In particular:

- adapters cannot import repositories or provider SDKs;
- Operations cannot import transport frameworks;
- application/domain code cannot import concrete providers or persistence;
- workers and schedulers must invoke registered Operations;
- central and edge profiles cannot own duplicate business implementations;
- provider-specific types cannot appear in canonical contracts.

## Test strategy

`docs/testing/operation-invocation-test-plan.md` defines contract, fixture, multi-adapter, context, authorization, idempotency, transaction, effect, error, realization-equivalence, observability and architecture tests.

## Consequences

- Business meaning exists in one canonical Operation contract.
- All delivery adapters share semantics and tests.
- Workers and schedulers do not create private service contracts.
- Operation metadata drives authorization, observability, Economics by Design and capability discovery.
- Runtime APIs remain smaller than the application model and provider-neutral.
- Provider replacement or routing does not create a new consumer-facing Operation.
- Native and external realizations pass shared semantic conformance evidence.
- Async, cancellation, idempotency and indeterminate outcomes become explicit design responsibilities.

## Rejected alternatives

- Controller/service-first architecture: couples behavior to transport conventions.
- Separate central and edge services: duplicates semantics.
- Provider-specific Operations such as `CallRGK`: leaks realization identity into the business contract.
- Separate sync and async business interfaces: creates semantic duplication.
- Untyped maps as the canonical runtime contract: destroys compile-time and compatibility guarantees.
- Generic service locator available to business code: hides dependencies and weakens architecture tests.
- Direct adapter-to-provider or adapter-to-repository calls: bypass canonical semantics and cross-cutting controls.
- Adapter-owned transactions or event publication: splits business atomicity across delivery mechanisms.
- Mandatory dynamic plugins for internal composition: adds complexity before a proven extension requirement.

## Design evidence

- `docs/architecture/canonical-operation-contract.md`;
- `governance/schemas/operation-manifest.schema.json`;
- `governance/examples/create-basket.operation.json`;
- `docs/testing/operation-invocation-test-plan.md`;
- `docs/architecture/operation-architecture-enforcement.md`;
- ADR-0024 Governed Capability Realization integration;
- RFC-0002 Runtime Foundation integration.

## Implementation evidence required by M1

- one Operation invoked unchanged by at least two adapter forms;
- deadline and cancellation propagation proof;
- authorization, idempotency and transaction/effect enforcement;
- architecture tests preventing adapter-to-repository and adapter-to-provider bypass;
- shared fixtures and contract tests;
- Operation- and realization-level trace and economic correlation;
- the same Operation bound to native and delegated realizations where required by ADR-0024.

This ADR can be accepted as an architectural decision before executable M1 evidence exists. Gate #54 must not claim those implementation proofs until they are produced.