# ADR-0024 — Governed Capability Realization

- Status: Reviewable for acceptance
- Date: 2026-07-18
- Governing issue: #68
- Parent epic: #46
- Related: ADR-0021, RFC-0002, #47, #48, #49, #51, #52, #53

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

A realization may expose provider-specific limitations through a manifest, but it may not redefine canonical business meaning silently. Provider payloads, SDK types and transport errors are translated at the adapter boundary.

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

The normative machine-readable structure is `governance/schemas/capability-realization-manifest.schema.json`; manifests are validated before they become eligible for binding.

## Binding and routing

Binding is explicit, versioned, observable and auditable. It may be static at build/deployment time or dynamically selected from approved manifests and policy. It must never be an unrestricted service locator.

Routing may select by tenant, country, channel, store, device class, runtime profile or capability revision. Cost-aware or availability-aware selection is allowed only inside declared semantic, authority, security and quality constraints. Economic optimization cannot weaken correctness silently.

The binding contract, precedence and rejection taxonomy are defined in `docs/architecture/capability-binder-contract.md`.

## Composition and pipelines

Whole-Operation delegation is the default. Bounded composition is allowed only when the semantic contract defines a governed graph. Every stage declares:

- whether it enriches, validates, calculates, authorizes, persists, aggregates or finalizes;
- the authority it has over the final outcome;
- ordering and determinism requirements;
- failure, compensation and partial-result behavior;
- trace and economic correlation.

A pipeline is not an arbitrary hook chain. Stages must be capability-governed, schema-declared and conformance-tested.

## Runtime relationship

```text
Delivery adapter
  -> canonical Operation
    -> capability-oriented port
      -> Capability Binder
        -> Native realization
        -> Delegated provider adapter
        -> Governed composition/pipeline
```

Delivery adapters do not select or call providers. Realizations do not bypass authorization, observability, evidence, transaction, authority or economic controls.

## Failure, offline and authority semantics

Remote execution is not treated as a transparent local method. Every realization explicitly defines deadlines, cancellation, retry, idempotency, duplicate handling, partial and indeterminate outcomes, reconciliation, fallback and offline behavior.

Fallback is allowed only when semantic meaning, required authority, security, legal/fiscal obligations and accepted quality constraints remain valid. The normative decision tables and evidence expectations are defined in `docs/architecture/capability-realization-semantics-and-evidence.md`.

## Initial proving cases

The model must be proven without making any provider mandatory:

1. commercial calculation via a native realization and an RGK-compatible adapter;
2. customer enquiry via a delegated external provider;
3. invoice issuance via an external authoritative system;
4. one composed pipeline: native validation -> delegated commercial calculation -> native fiscal/rounding validation -> canonical finalization.

These are proving cases, not an exhaustive list of delegable capabilities.

## Consequences

- Any present or future capability can be realized internally or externally without changing its canonical consumer contract.
- Existing specialist platforms can participate as integral, governed parts of UC Rust.
- Provider replacement becomes a binding, compatibility and conformance problem rather than a domain rewrite.
- Runtime metadata supports compatibility, observability, resilience and Economics by Design decisions.
- Native/delegated equivalence is judged on canonical meaning, not provider implementation identity.
- Architecture and testing become more demanding because remote failure, partial authority and provider differences remain explicit.

## Rejected alternatives

- Provider-specific Operations such as `CallRGK`: leaks implementation identity into business contracts.
- Direct adapter-to-provider calls: bypass canonical semantics and cross-cutting controls.
- Generic runtime hooks: create hidden behavior and ungoverned ordering.
- Pretending remote calls behave like local methods: hides latency, partial failure and indeterminate outcomes.
- Selecting providers primarily by cost: may weaken semantic, legal or quality guarantees.
- Mandatory dynamic binary plugin ABI for 1.0: unnecessary for network/process-based delegation and premature to stabilize.

## Design evidence

- `docs/architecture/governed-capability-realization.md`;
- `docs/architecture/capability-binder-contract.md`;
- `docs/architecture/capability-realization-semantics-and-evidence.md`;
- `governance/schemas/capability-realization-manifest.schema.json`;
- `tools/validate_capability_realization_manifests.py`;
- ADR-0021 integration;
- RFC-0002 integration;
- CR-0002 Pricing integration;
- 1.0 roadmap and gate integration.

## Implementation evidence required by M1/M3

- native and delegated implementations passing the same semantic conformance fixtures;
- binding/routing tests across at least two policy dimensions;
- failure, timeout, retry, idempotency and indeterminate-outcome tests;
- offline/fallback proof;
- provider compatibility and rollout/rollback proof;
- one composed/pipelined executable proof;
- operation-level trace, provider attribution and economic evidence;
- architecture tests preventing provider leakage and adapter bypass.

This ADR can be accepted as an architectural decision before executable M1/M3 evidence exists; the implementation gate must not claim those proofs until they are produced.
