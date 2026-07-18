# Charter Amendment 001 — Governed Capability Realization

- Status: Accepted direction; architecture evidence pending
- Parent charter: `PROJECT_CHARTER.md`
- Governing issue: #68
- Governing decision: ADR-0024
- Date: 2026-07-18

## Amendment

UC Rust adopts Governed Capability Realization as a platform characteristic.

> UC Rust owns semantic authority, not implementation exclusivity. A capability is defined by its semantic contract. Its realization may be native, delegated, composed, pipelined or hybrid, provided that semantic invariants, authority rules and accepted quality budgets remain preserved.

This principle applies to any current or future capability. Pricing, loyalty, customer enquiry, invoicing, payment, tax, inventory, fulfillment, search, notification, fraud and specialized AI are examples only.

## Scope consequence

The 1.0 foundation must provide a canonical Capability → Operation → Realization model, governed binding, realization manifests, semantic conformance tests, explicit authority/failure/offline semantics, compatibility, observability and economic attribution.

The project does not commit to implementing every capability natively. A deferred native capability may participate through an accepted external realization without weakening semantic governance.

## Architectural consequence

- consumers invoke canonical Operations;
- delivery adapters never call providers directly;
- provider contracts and SDK types remain outside the application/domain contracts;
- native and external realizations are tested against the same semantic fixtures;
- composed and pipelined realizations use explicit governed stage graphs, not arbitrary hooks;
- realization selection may consider tenant, jurisdiction, channel, store, runtime profile, compatibility, availability and economics only within accepted semantic and quality constraints.

## Change-control disposition

This amendment generalizes the charter's existing capability-delegation language. It does not make RGK, Koncentro or any other provider mandatory. It creates a new P0 architecture obligation under #68 and adds ADR-0024 to the acceptance path for the target architecture and runtime foundation.
