# UC Rust Target Architecture

- Status: Draft for approval
- Governing issue: #12
- Related decisions: ADR-0002, ADR-0005 through ADR-0016
- Normative source: UC-BoK
- Last updated: 2026-07-18

## Architectural intent

UC Rust is a modular, distributed Unified Commerce reference implementation. It provides one canonical domain and application core that can be composed into central and edge runtime profiles, exposed through multiple delivery adapters and connected to replaceable infrastructure providers.

The architecture separates three planes:

1. **Business plane** — domain types, invariants and canonical application operations.
2. **Data plane** — central and edge runtimes, APIs, persistence, event delivery and synchronization.
3. **Control plane** — identity, fleet inventory, desired/actual state, configuration, deployment, monitoring, diagnostics and recovery.

## Logical view

```text
Touchpoints and integrations
REST | gRPC | Mobile | POS | SCO | Workers | Jobs | CLI | Devices
                         |
                         v
Inbound adapters and contract mapping
                         |
                         v
Canonical application operations
                         |
        +----------------+----------------+
        |                                 |
        v                                 v
Domain model and policies          Outbound capability ports
                                          |
                 +------------------------+-----------------------+
                 |              |             |          |        |
                 v              v             v          v        v
             Persistence       Cache         Storage    AuthZ   External systems
             PG / SQLite       Redis         S3 / FS   SpiceDB  ERP/OMS/CRM/PSP
```

No delivery or infrastructure adapter owns business procedures. Every behavior is implemented once in the application/domain core.

## Deployment profiles

### Central runtime

Provides the complete configured set of Unified Commerce capabilities, global orchestration, cross-location views, integration endpoints, fleet control and convergence authority where assigned.

### Store-edge runtime

Provides an explicitly declared subset of the same application operations and API semantics. It persists required replicas and edge-authoritative work locally and remains operational through supported WAN failures.

### Warehouse-edge runtime

Candidate profile for local fulfillment, inventory and operational workflows. It must reuse the same application operations and declare different capability/data authority profiles where necessary.

### Mobile, SoftPOS and smart-checkout surfaces

Delivery profiles or adapters over stable application contracts. They must not fork business semantics. Provider/device-specific behavior remains outside the core.

## Capability model

Every capability declares:

- UC-BoK identifier;
- domain/application owner;
- exposed operations;
- supported runtime profiles;
- offline classification;
- source/authority of required data;
- consistency and freshness guarantees;
- authorization vocabulary;
- performance/SLO budget;
- contract version;
- migration and compatibility rules.

Allowed offline classifications:

- `offline-capable`
- `offline-capable-with-limits`
- `read-only-offline`
- `degraded-offline`
- `online-required`
- `forbidden-offline`

## Data categories

### Cache

Reconstructible optimization. It may be discarded and repopulated. Its loss cannot remove a committed business effect.

### Central-authoritative replica

Versioned local representation of centrally governed information such as product, price or configuration snapshots. It carries source revision, checkpoint, validity and freshness metadata.

### Edge-authoritative state

Business effects created locally while the edge owns the operation, such as a sale or durable outbox record. It must survive restart, prolonged WAN outage and update interruption.

### Converged/derived view

Read model composed from multiple authorities. It must not be mistaken for a transaction authority.

## Synchronization architecture

Synchronization is a first-class platform capability, not table replication.

Central-to-edge supports:

- baseline snapshots;
- ordered deltas;
- checkpoints and resumable delivery;
- signature/checksum verification;
- expiry and freshness policy;
- schema and protocol negotiation.

Edge-to-central supports:

- local transactional outbox;
- globally unique event/operation identifiers;
- edge identity and monotonic local sequence;
- idempotent publication and acknowledgement;
- retry, duplicate and out-of-order handling;
- reconciliation and dead-letter disposition.

Conflict policy is defined per business object/operation using explicit strategies such as single-writer authority, append-only merge, semantic merge, compensation, reject/escalate or governed manual reconciliation. Generic last-write-wins is forbidden unless an ADR demonstrates semantic safety.

## Contract architecture

REST, gRPC, event and serialized contracts are versioned separately from implementations. Central and edge endpoints preserve the same semantics where a capability is supported. Runtime capability discovery communicates availability or limitations without changing operation meaning.

Canonical, human-readable fixtures provide executable examples and are reused in tests and documentation.

## Identity and authorization

Authentication is OAuth 2.0/OpenID Connect provider-neutral. Authorization is Zanzibar-style and deny-by-default through an application-owned port, initially implemented by SpiceDB centrally.

Offline/edge authorization must explicitly define:

- locally verifiable identity evidence;
- replicated relationships or policy material;
- decision freshness and revocation exposure;
- high-risk operations that remain online-required;
- audit evidence and reconciliation.

Authorization failure never silently becomes allow.

## Persistence and infrastructure

Domain/application crates never depend on SQL, Redis, S3, SpiceDB or vendor SDK types. PostgreSQL, SQLite, cache, storage and external services are adapters behind capability-oriented ports. Every implementation passes shared contract tests.

Database migrations are provider-specific, ordered, immutable and forward-only. Central and edge rolling compatibility uses expand/migrate/contract.

## Edge fleet control plane

The central control plane owns desired state; each edge owns local reconciliation and reports actual state.

The compatibility envelope includes:

- application release;
- runtime profile;
- central/edge compatibility range;
- database schema;
- sync protocol;
- public contract versions;
- configuration schema/revision;
- authorization schema;
- UC-BoK revision;
- hardware/OS requirements.

Updates are pull-based, signed, content-addressed, resumable, staged, preflight-checked and rollback-capable. Canary/cohort rollout uses technical, synchronization and retail-operational health gates.

## Observability and operational health

Technical health includes process, CPU, memory, disk, network, DB, queue and certificate state.

Retail-operational health includes ability to sell, price/promotion validity, payment availability, fiscal readiness, local persistence durability, outbox safety and synchronization lag.

`offline-operational` is a valid healthy retail state even though central connectivity is absent.

## AI architecture

AI is an optional capability layer over governed application contracts and evidence. Models and providers remain replaceable. AI does not bypass authorization, domain invariants or audit. Critical decisions require explicit human-in-the-loop or bounded autonomous policy, evaluation datasets, fallback and performance/error budgets.

## Extension architecture

Retailer/vendor extensions integrate through:

- versioned public contracts;
- outbound/inbound ports;
- event subscriptions;
- UI extension boundaries outside the application core;
- capability registration and permission declarations;
- compatibility and performance tests.

Extensions cannot link directly to internal persistence or bypass canonical application operations.

## Initial bounded capabilities

1. Basket
2. Pricing/commercial calculation
3. Checkout
4. Order
5. Identity and authorization
6. Synchronization
7. Edge fleet control
8. Platform configuration and observability

Customer, loyalty, inventory, fulfillment, smart checkout, payments, AI and analytics enter through explicit roadmap issues and UC-BoK traceability.

## Enforcement

Architecture is enforced through:

- dependency and forbidden-import tests;
- contract suites for adapters;
- operation ownership manifest;
- UC-BoK traceability validation;
- central/edge capability matrix;
- schema/contract compatibility checks;
- partition and reconciliation tests;
- signed fleet update proof;
- quality model evidence consumed by Project Ready #19.
