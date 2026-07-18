# UC Rust product glossary

This glossary defines the project-level terms used by the UC Rust charter and roadmap. UC-BoK remains authoritative for Unified Commerce semantics; where a stable UC-BoK identifier is not yet available, the term is provisional and must be reconciled through #38.

## Core terms

### Capability
A coherent Unified Commerce responsibility that delivers a business outcome and has an explicit owner, contract, quality expectations and evidence. A capability is not synonymous with a crate, service or deployment unit.

### Operation
The canonical application-level implementation of one business use case. Delivery adapters invoke Operations; they do not own business procedures.

### Central runtime
A deployment profile responsible for globally coordinated capabilities, shared integrations, cross-location processing and fleet-level control-plane concerns.

### Store-edge runtime
A deployment profile running selected capabilities close to store operations, including explicitly classified offline behavior, local authority where granted, and governed convergence with central systems.

### Runtime profile
A declared composition of capabilities, adapters, configuration and operational guarantees for a deployment context. Profiles reuse the same domain and application contracts.

### Touchpoint
A user- or device-facing interaction surface such as fixed POS, mobile POS, self-checkout, API client, worker or CLI. A touchpoint is a delivery adapter, not a separate source of business truth.

### Data authority
The declared owner permitted to create or decide a class of state for a defined scope and time. Authority is distinct from replication, caching or physical storage location.

### Offline class
A governed classification of what a capability may do during loss or degradation of connectivity, including authority, freshness, risk and convergence obligations.

### Convergence
The verified process by which distributed state reaches an accepted consistent outcome after disconnection, retry, duplication, reordering or version skew, without silent loss or duplication of business effects.

### Provider boundary
A stable capability-oriented port isolating specialist systems such as PSPs, identity providers, fiscal services, devices, ERP, CRM, OMS or AI providers from the application core.

### Reference implementation
Executable evidence that UC-BoK concepts and constraints can be implemented coherently. UC Rust may expose ambiguity or propose changes, but does not silently redefine UC-BoK.

### Evidence bundle
The accepted functional, architectural, distributed, security, operational and economic proof required by a release gate.

### Economic Unit
The stable unit to which technical consumption and cost are attributed, such as an Operation invocation, store/day or tenant/month.

### Fleet control plane
The capabilities that identify, inventory, configure, deploy, monitor and recover distributed runtime nodes without owning retail business procedures.

## Scope qualifiers

- **Implemented** — delivered and supported by UC Rust with accepted evidence.
- **Integrated** — provided through a governed external provider boundary.
- **Planned** — within the 1.0 roadmap but not yet accepted as implemented.
- **Deferred** — intentionally outside the 1.0 release scope.
- **Candidate** — under evaluation and not a commitment.
- **Intentionally divergent** — differs from UC-BoK through an explicit reciprocal accepted decision.
