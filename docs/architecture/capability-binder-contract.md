# Capability Binder Contract

- Status: Reviewable baseline
- Governing decision: ADR-0024
- Governing issue: #68

## Purpose

The Capability Binder resolves a canonical Operation invocation to one approved Capability Realization without exposing provider identity to delivery adapters or business code.

## Inputs

A binding decision receives only governed metadata:

- capability and Operation identifiers plus semantic revision;
- tenant, legal entity, country and fiscal jurisdiction;
- channel, touchpoint, store and device class;
- runtime profile and connectivity state;
- required authority and outcome state;
- security, residency and data-minimization constraints;
- deadline, offline class and compatibility envelope;
- accepted quality and economic budgets;
- binding-policy revision.

Business payloads are not used for provider selection unless the capability contract explicitly declares a non-sensitive routing discriminator.

## Output

The Binder returns a `BindingDecision` containing:

- selected realization identifier and manifest revision;
- selected Operation semantic revision;
- policy revision and deterministic decision reason;
- authority role and expected outcome state;
- invocation mode, deadline and retry policy;
- offline/fallback disposition;
- compatibility evidence reference;
- observability and economic attribution dimensions.

It may instead return a stable canonical rejection such as `NoEligibleRealization`, `AuthorityMismatch`, `CompatibilityMismatch`, `PolicyDenied`, `OfflineUnsupported`, `BudgetUnsatisfied` or `SecurityConstraintUnsatisfied`.

## Selection algorithm

1. Load only approved, non-withdrawn manifests compatible with the runtime and binding-policy revision.
2. Filter by capability, Operation and semantic revision.
3. Filter by runtime profile, invocation mode and connectivity state.
4. Enforce tenant, jurisdiction, residency, authorization and data-minimization constraints.
5. Enforce required authority and outcome-state semantics.
6. Enforce deadline, offline, idempotency and compatibility requirements.
7. Enforce accepted quality budgets.
8. Apply explicit routing priority.
9. Use economic preference only among semantically and operationally equivalent eligible realizations.
10. Emit the decision and its evidence.

No later step may re-admit a realization rejected by an earlier semantic, authority, security or compatibility constraint.

## Routing precedence

Precedence is explicit and versioned:

1. legal/fiscal mandate;
2. tenant-specific binding;
3. legal entity or country binding;
4. store/channel/touchpoint binding;
5. runtime-profile and connectivity binding;
6. default capability binding;
7. approved fallback chain.

Equal-precedence rules that select different realizations are invalid configuration, not nondeterministic routing.

## Whole Operation and bounded stages

The default unit of binding is a whole canonical Operation. Stage-level binding is allowed only when the capability specification defines a governed semantic graph. Each stage must have a stable identifier, typed boundary, authority contribution, failure policy and evidence obligations.

Provider-defined hooks and arbitrary callbacks are forbidden.

## Authority rules

A binding must distinguish:

- source-data authority;
- calculation or decision authority;
- legal issuance authority;
- persistence/system-of-record authority;
- final versus advisory outcome.

A fallback is eligible only when it preserves the required authority. For example, a local pricing snapshot may replace a remote calculator only for an Operation and jurisdiction that explicitly permit that authority and staleness profile.

## Determinism and change

The same governed context, manifest set and policy revision must produce the same decision. Availability-aware routing may change the selected realization only through an explicit policy branch, and the resulting reason must be observable.

Bindings are immutable for the lifetime of one invocation. Long-running Operations record the binding and compatibility envelope used when work began.

## Failure isolation

Each delegated realization has:

- independent concurrency limits and bulkhead;
- circuit-breaker policy;
- deadline budget;
- retry budget;
- protected provider diagnostics;
- rollout and rollback state.

Failure of one provider must not exhaust unrelated capability or tenant budgets.

## Security boundary

The Binder does not issue unrestricted provider clients. It returns an invocation handle constrained to the selected Operation, tenant, deadline and policy. Credentials remain inside the provider adapter. Provider SDK and payload types cannot cross into canonical domain/application contracts.

## Observability and Economics by Design

Every decision and invocation records, without sensitive payloads:

- capability, Operation and semantic revision;
- realization, provider and manifest revision;
- policy revision and decision reason;
- tenant/store/profile dimensions where permitted;
- latency, retries, failure disposition and circuit state;
- resource consumption and cost attribution unit;
- fallback or reconciliation usage.

## Contract tests

The Binder test suite must prove:

- deterministic selection;
- precedence and conflict rejection;
- semantic-version and compatibility filtering;
- authority-preserving fallback;
- security and residency rejection;
- two-dimensional routing, at minimum tenant plus runtime profile;
- cost-aware selection never bypasses higher-order constraints;
- no adapter-to-provider bypass and no provider type leakage.
