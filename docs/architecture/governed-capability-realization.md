# Governed Capability Realization

- Status: Design baseline
- Governing decision: ADR-0024
- Governing issue: #68
- Related: ADR-0021, #47, #48, #49, #51, #52

## Purpose

This specification defines how UC Rust treats internal and external implementations as interchangeable, governed realizations of canonical capabilities and Operations.

The model is intentionally capability-agnostic. Pricing, loyalty, customer enquiry and invoicing are initial examples, not a closed list.

## Core model

```text
Capability
  contains one or more Operations

Operation
  defines canonical semantic input, output, errors and invariants

Capability Realization
  fulfills one or more Operations under a declared manifest

Capability Binding
  selects an approved realization for a governed execution context
```

### Fundamental rule

Consumers integrate with canonical Operations, not realization identities.

```text
Correct:  CalculateCommercialOutcome(command)
Wrong:    CallRGK(payload)
```

## Realization kinds

| Kind | Meaning | Typical form |
|---|---|---|
| Native | Application-owned implementation | Rust crate/component |
| Delegated | External system owns execution | remote service, sidecar, local process |
| Composed | Multiple realizations jointly produce one semantic result | aggregator/orchestrator |
| Pipeline | Ordered governed stages transform or validate an outcome | enrichment, calculation, fiscal stage |
| Hybrid | Binding changes by profile/connectivity/policy | edge local plus central remote |

These kinds describe execution topology, not semantic ownership.

## Capability Binder

The Capability Binder resolves an Operation to an eligible realization. Inputs may include:

- tenant and legal entity;
- country or fiscal jurisdiction;
- channel, touchpoint and store;
- central, store-edge or offline runtime profile;
- capability and Operation revision;
- provider availability and compatibility;
- security, residency and compliance constraints;
- accepted latency and economic budgets.

Selection must be deterministic for the same governed context or explain why a policy-driven change occurred. Every decision emits the selected realization, policy revision and reason without exposing sensitive data.

The Binder is not available as an unrestricted service locator. Application Operations request capability ports; the runtime owns binding.

## Provider and realization manifest

A realization manifest is versioned and signed where it participates in managed deployment or fleet configuration.

```yaml
realization_id: rgk-commercial-calculation-eu-v1
provider_id: retex-iconic-rgk
capability_id: pricing
operations:
  - id: calculate-commercial-outcome
    semantic_versions: [1.0]
kind: delegated
execution:
  transport: grpc
  profiles: [central, store-edge-online]
authority:
  role: authoritative-calculator
  owns_source_data: false
guarantees:
  idempotency: invocation-key
  deadline_ms: 250
  retry: safe-before-ack
  offline: unavailable
compatibility:
  provider_contract: rgk-engine-v1
  adapter_revision: 1.0.0
quality:
  latency_budget_ms: 200
  availability_target: 99.9
economics:
  attribution_unit: calculation
security:
  tenant_isolation: required
  data_minimization_profile: commercial-calculation-v1
```

The concrete schema will be governed separately and validated automatically.

## Authority model

For every Operation and realization, the following must be explicit:

1. who is authoritative for source data;
2. who is authoritative for the calculated or issued outcome;
3. whether the result is provisional, reserved, final or advisory;
4. what evidence proves that authority;
5. how conflicts and stale data are surfaced;
6. whether another realization may replace or amend the result.

Delegation does not imply authority transfer automatically. For example, an external engine may calculate promotions while UC Rust remains authoritative for basket state; an external fiscal platform may instead be authoritative for invoice identity and legal issuance evidence.

## Invocation semantics

Every realization uses the canonical invocation envelope and must declare:

- deadline and cancellation behavior;
- idempotency key scope and retention;
- safe and unsafe retry points;
- duplicate detection;
- partial and indeterminate outcome semantics;
- compensation or reconciliation requirements;
- correlation, audit and economic attribution.

Provider-specific failures are mapped to the stable canonical taxonomy while retaining provider diagnostics in protected operational evidence.

## Offline and fallback

Fallback is never implicit. Each binding declares one of:

- **block** — Operation is unavailable safely;
- **local realization** — switch to an approved edge/native realization;
- **stale snapshot** — execute within a declared freshness bound;
- **queue/defer** — persist intent for later execution where semantics allow it;
- **degraded result** — allowed only when represented explicitly in the canonical outcome;
- **manual resolution** — create an auditable operational disposition.

A cheaper or available realization cannot be selected if it changes authority or weakens legal, fiscal, security or correctness guarantees.

## Composition and pipeline rules

A composed or pipelined realization has a declared graph. Each node defines:

- canonical stage identifier;
- semantic role;
- allowed inputs and outputs;
- authority contribution;
- ordering and concurrency rules;
- timeout and failure policy;
- compensation/reconciliation behavior;
- resource and economic budget.

No arbitrary callback, reflection hook or provider-defined code may inject behavior into the application core.

Example:

```text
CalculateCommercialOutcome
  -> internal eligibility/context validation
  -> RGK commercial calculation
  -> internal fiscal/rounding validation
  -> canonical PricingOutcome
```

## Conformance model

Every realization of the same Operation must pass a shared semantic conformance kit containing:

- canonical positive and negative fixtures;
- invariants and property tests;
- stable error/disposition expectations;
- idempotency and replay scenarios;
- timeout, retry and indeterminate-outcome scenarios;
- security and tenant-isolation tests;
- compatibility and migration tests;
- trace and economic evidence assertions.

Provider-specific capabilities may be narrower than the canonical contract. Unsupported combinations must be declared in the manifest and rejected during binding or validation, never discovered as silent semantic drift.

## Initial proving cases

### Commercial calculation

A basket Operation can bind to a native realization or an adapter for `retex-iconic/rgk`. Both produce the canonical commercial outcome and explanation model.

### Customer enquiry

The canonical enquiry Operation may bind to Koncentro or another provider. Provider customer models remain outside the canonical contract.

### Invoice issuance

An external ERP or fiscal system may own legal issuance. UC Rust records the canonical request, authority disposition, external identity and evidence without pretending to own the legal document lifecycle.

### Future capabilities

The same model applies without architectural change to inventory, order sourcing, tax, fraud, payments, AI, product data, fulfillment, notification, search or capabilities not currently anticipated.

## 1.0 boundary

UC Rust 1.0 must establish:

- the canonical realization abstraction;
- manifest and binding concepts;
- one native/delegated equivalence proof;
- explicit failure/offline behavior;
- conformance fixtures;
- observability and economic attribution.

It does not require a stable third-party binary ABI, unrestricted dynamic plugins or implementation of every proving provider.
