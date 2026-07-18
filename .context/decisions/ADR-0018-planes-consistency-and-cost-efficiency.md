# ADR-0018 — Architectural planes, data consistency and cost-efficient component selection

- Status: Accepted
- Date: 2026-07-18
- Governing issues: #29, #31, #39, #40, #42

## Context

UC Rust runs the same application model across central and peripheral runtimes, but distributed retail introduces different concerns that must not be collapsed into one mechanism. Application deployment at the edge, business-data replication and distributed coordination have different authority, availability and failure semantics. Treating them as one generic distributed-systems problem would encourage unnecessary consensus, hidden coupling and excessive cost-to-serve.

Component choices also affect recurring infrastructure cost, edge hardware requirements, network consumption, operational effort and incident burden. Performance and cost efficiency must therefore be architectural constraints rather than late optimizations.

## Decision

### 1. Separate architectural planes

UC Rust distinguishes four planes:

1. **Application plane** — canonical domain rules and application operations. Business behavior is implemented once and remains independent from deployment topology.
2. **Data plane** — business-data authority, persistence, replication, synchronization, ordering, deduplication, freshness, reconciliation and conflict handling.
3. **Control plane** — desired state, software deployment, configuration, fleet inventory, health, rollout, rollback and drift reconciliation.
4. **Coordination plane** — leader election, leases, distributed locks, quorum decisions, replicated logs and consensus where a shared decision is genuinely required.

A component may participate in more than one plane only through explicit interfaces and documented guarantees. Plane boundaries must remain observable in architecture, ownership, failure handling and cost accounting.

### 2. Deployment state is not business-data consistency

The control plane is authoritative for desired deployment and configuration state. Edge nodes reconcile desired state safely when connectivity and local operational conditions permit. WAN unavailability may delay deployment but must not corrupt local business data or stop supported offline operations.

Business data follows data-class-specific authority and consistency rules. A software version, configuration revision or rollout acknowledgement must never be reused as an implicit business-data consistency mechanism.

### 3. No universal consistency model

Every synchronized data class must declare:

- stable data-class identifier and business owner;
- authoritative writer or writers;
- allowed write locations and offline behavior;
- consistency model;
- freshness or staleness budget;
- ordering scope;
- idempotency and deduplication identity;
- conflict-detection and resolution policy;
- reconciliation and recovery behavior;
- retention, residency and privacy classification;
- expected volume, latency, storage and network budgets.

Allowed consistency models include, when explicitly justified:

- single-authority strongly consistent;
- single-authority asynchronously replicated;
- causal or read-after-write consistency;
- per-source ordered eventual consistency;
- bounded-staleness read model;
- escrow or reservation-based limited offline writes;
- online-required operations;
- explicitly governed convergent data types.

Generic last-write-wins and implied global ordering are forbidden.

### 4. Consensus is exceptional

UC Rust does not implement or require application-wide Paxos, Raft or global consensus.

Consensus or quorum coordination is introduced only when all of the following are documented:

- the shared decision that cannot be safely partitioned by authority;
- the required safety and liveness properties;
- the expected partition behavior;
- the operational and cost impact;
- why database transactions, ownership partitioning, idempotency, leases or an existing infrastructure capability are insufficient.

Prefer proven infrastructure-provided consensus rather than implementing a new protocol in UC Rust. Provider consensus does not remove the need to define application authority and failure semantics.

### 5. Cost-to-serve is a first-class quality attribute

Every significant architectural component or managed service requires a lightweight decision record containing:

- required guarantees and workload assumptions;
- minimum viable topology;
- steady-state and peak CPU, memory, disk, I/O and network expectations;
- central and per-edge footprint;
- horizontal scaling unit;
- storage growth and retention drivers;
- licence, managed-service and data-transfer costs where applicable;
- operational burden, upgrade effort and failure-recovery complexity;
- alternatives considered and the reason the selected option is the least costly solution that satisfies the guarantees.

Selection order is:

1. remove the component if the capability is unnecessary;
2. reuse an existing platform capability when guarantees and isolation are adequate;
3. prefer a simpler embedded or shared component when blast radius and tenancy remain acceptable;
4. introduce a dedicated distributed component only when measurements or required guarantees justify it.

### 6. Performance and efficiency budgets

Critical paths and deployment profiles must define measurable budgets. At minimum:

- p50, p95 and p99 latency;
- throughput and concurrency;
- CPU and memory per central instance and edge node;
- persistent storage and daily growth;
- bytes transferred per business operation and per synchronization cycle;
- idle footprint;
- backlog recovery throughput and time;
- cost allocation unit, such as tenant, store, terminal, order or transaction.

Claims of lower cost or higher performance require reproducible benchmark evidence using representative workloads. Unit-cost regressions must be visible even when functional correctness remains unchanged.

## Consequences

- Fleet deployment and business-data consistency evolve independently behind explicit contracts.
- The synchronization model remains per data class rather than becoming an accidental global event log.
- Consensus becomes a reviewed exception rather than a default architectural reflex.
- Component selection must expose recurring cost and operational complexity before adoption.
- Simple embedded adapters are preferred where they satisfy guarantees, while durable distributed infrastructure remains available when justified.
- The quality model and architecture review must reject components without authority semantics, measurable budgets or a cost-to-serve rationale.
