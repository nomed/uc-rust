# ADR-0024 — Governed Capability Realization

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #68
- Parent epic: #46
- Related: ADR-0021, #47, #48, #49, #51, #52, #53

## Context

UC Rust defines canonical Unified Commerce capabilities and Operations, but a retailer may already rely on specialist systems for commercial calculation, loyalty, customer enquiry, invoicing, fiscalization, payment, inventory, fraud, tax, AI or capabilities not yet anticipated.

Treating every such system as a special integration would duplicate orchestration rules and leak provider semantics into delivery adapters and application code. Treating them only as binary plugins would also be too narrow: a realization may be native, in-process, sidecar-based, edge-local, remote, composed or pipelined.

## Decision

UC Rust adopts **Governed Capability Realization** as a first-class platform model.

> UC Rust owns semantic authority, not implementation exclusivity. A capability is defined by its semantic contract. Its realization may be native, delegated, composed or pipelined, provided that semantic invariants, authority rules and accepted quality budgets remain preserved.

The canonical model is:

```text
Capability
  -> Operation
    -> Capability Realization
```

A realization may be:

- **Native** — implemented inside the UC Rust application boundary;
- **Delegated** — fulfilled by an external provider through a governed adapter;
- **Composed** — assembled from multiple realizations with an explicit semantic composition rule;
- **Pipeline** — executed as ordered stages with declared inputs, outputs, authority and failure behavior;
- **Hybrid** — a governed combination varying by runtime profile, connectivity or policy.

The caller invokes the canonical Operation and never invokes a provider-specific API directly. A Capability Binder selects the eligible realization using governed policy such as tenant, country, channel, store, runtime profile, capability revision, availability, compliance and accepted economic constraints.

## Semantic authority

Every capability owns:

- stable Operation identifiers;
- canonical typed inputs, outputs and errors;
- invariants and outcome states;
- authority and source-of-truth rules;
- idempotency and indeterminate-outcome semantics;
- security, privacy and audit obligations;
- compatibility and evidence requirements.

A realization may expose provider-specific limitations through a manifest, but it may not redefine the canonical business meaning silently. Provider payloads, SDK types and transport errors are translated at the adapter boundary.

## Realization manifest

Each realization declares at minimum:

- provider and realization identity;
- supported capability and Operation revisions;
- execution form and supported runtime profiles;
- authority role and data ownership;
- latency, availability, resource and economic budgets;
- online/offline behavior and freshness limits;
- idempotency, retry, deadline, cancellation and recovery guarantees;
- security, tenant isolation and data-minimization constraints;
- compatibility envelope, rollout and rollback metadata;
- conformance evidence and operational health signals.

## Binding and routing

Binding is explicit, versioned, observable and auditable. It may be static at build/deployment time or dynamically selected from approved manifests and policy. It must never be an unrestricted service locator.

Routing may select by tenant, country, channel, store, device class, runtime profile or capability revision. Cost-aware or availability-aware selection is allowed only inside declared semantic, authority, security and quality constraints. Economic optimization cannot weaken correctness silently.

## Composition and pipelines

Whole-Operation delegation is the simplest form, but bounded composition is allowed when the semantic contract defines it. Every stage must declare:

- whether it enriches, validates, calculates, authorizes, persists or finalizes;
- the authority it has over the final outcome;
- ordering and determinism requirements;
- failure, compensation and partial-result behavior;
- trace and economic correlation.

A pipeline is not an arbitrary hook chain. Stages must be capability-governed and conformance-tested.

## Runtime relationship

```text
Delivery adapter
  -> canonical Operation
    -> Capability Binder
      -> Native realization
      -> Delegated provider adapter
      -> Governed composition/pipeline
```

Delivery adapters do not select or call providers. Realizations do not bypass authorization, observability, evidence, transaction, authority or economic controls.

## Initial proving cases

The model must be proven without making any provider mandatory:

1. commercial calculation via a native realization and an RGK adapter;
2. customer enquiry via a delegated external provider;
3. invoice issuance via an external authoritative system;
4. one composed or pipelined example demonstrating explicit stage authority and failure semantics.

These are proving cases, not an exhaustive list of delegable capabilities.

## Consequences

- Any present or future capability can be realized internally or externally without changing its canonical consumer contract.
- Existing specialist platforms can participate as integral, governed parts of UC Rust.
- Provider replacement becomes a binding and conformance problem rather than a domain rewrite.
- Runtime metadata can support compatibility, observability, resilience and Economics by Design decisions.
- Architecture and testing become more demanding because remote failure, partial authority and provider differences must remain explicit.

## Rejected alternatives

- Provider-specific Operations such as `CallRGK`: leaks implementation identity into business contracts.
- Direct adapter-to-provider calls: bypass canonical semantics and cross-cutting controls.
- Generic runtime hooks: create hidden behavior and ungoverned ordering.
- Pretending remote calls behave like local methods: hides latency, partial failure and indeterminate outcomes.
- Mandatory dynamic binary plugin ABI for 1.0: unnecessary for network/process-based delegation and premature to stabilize.

## Required evidence

- capability realization specification and schema;
- native and delegated implementations passing the same semantic conformance fixtures;
- binding/routing tests across at least two policy dimensions;
- failure, timeout, retry, idempotency and indeterminate-outcome tests;
- offline/fallback decision table;
- provider compatibility and rollout/rollback proof;
- operation-level trace, provider attribution and economic evidence;
- architecture tests preventing provider leakage and adapter bypass.
