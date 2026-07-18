# Architectural planes, consistency and cost-to-serve

This document operationalizes ADR-0018 for architecture and component reviews.

## Plane model

| Plane | Owns | Must not silently own |
|---|---|---|
| Application plane | Domain invariants, use cases, canonical business behavior | Deployment orchestration, provider-specific replication or leader election |
| Data plane | Data authority, persistence, replication, synchronization, ordering, freshness and reconciliation | Software rollout desired state or implicit global consensus |
| Control plane | Fleet desired/actual state, deployment, configuration, health, rollout, rollback and drift | Business-data conflict resolution |
| Coordination plane | Leases, leadership, locks, quorum and consensus for explicitly shared decisions | General business workflows or every data write |

The planes may share transport and infrastructure, but ownership, APIs, failure behavior, observability and cost allocation remain explicit.

## Data-class consistency declaration

Every synchronized data class must have a declaration with these fields:

```yaml
data_class_id: stable.identifier
owner: accountable-team
authority:
  writers: [central | edge | named-system]
  partition_key: tenant/store/entity or none
writes:
  locations: [central, store-edge]
  offline_mode: offline-capable | limited | read-only | online-required
consistency:
  model: strong | async-replica | causal | per-source-ordered-eventual | bounded-staleness | escrow | governed-convergent
  freshness_budget: duration or not-applicable
  ordering_scope: entity | source | partition | global
identity:
  idempotency_key: definition
  deduplication_key: definition
conflicts:
  detection: rule
  resolution: reject | reconcile | domain-policy | manual
recovery:
  bootstrap: method
  replay: method
  reconciliation: method
data_governance:
  classification: value
  retention: duration
  residency: allowed-regions
cost_budgets:
  events_per_business_operation: number
  bytes_per_business_operation: number
  storage_growth_per_day: quantity
  backlog_recovery_target: duration
```

Missing fields mean the data class is not architecture-ready.

## Consensus decision test

Before introducing a consensus-capable component, answer all questions:

1. What exact shared decision requires one agreed value?
2. Can authority be partitioned by tenant, store, entity or workflow instead?
3. Can a database transaction, unique constraint, idempotency key or optimistic concurrency solve it?
4. Can a bounded lease solve it, and what happens during lease ambiguity?
5. What safety property must survive a network partition?
6. What availability is intentionally sacrificed under partition?
7. Which existing infrastructure already provides the required quorum semantics?
8. What are the node count, network, storage, operational and upgrade costs?

A missing answer blocks adoption. UC Rust does not implement a new consensus protocol without an accepted dedicated ADR or RFC.

## Component selection scorecard

Every material component decision compares at least the selected option and one simpler alternative.

| Dimension | Required evidence |
|---|---|
| Functional guarantees | Transactions, ordering, consistency, durability, tenancy and failure semantics |
| Performance | Representative p50/p95/p99, throughput and saturation point |
| Central footprint | CPU, memory, disk, IOPS and network at steady and peak load |
| Edge footprint | Idle and peak CPU/memory, disk growth, startup and constrained-hardware behavior |
| Scale unit | Cost and capacity per tenant, store, terminal, transaction or event |
| Network efficiency | Payload size, protocol overhead, retry amplification and egress |
| Operations | Deployment, monitoring, backup, restore, upgrade and incident effort |
| Availability cost | Minimum topology, replicas, quorum and cross-zone/region requirements |
| Licensing/service cost | Licence, managed service, support and data-transfer cost |
| Exit cost | Portability, migration path, data extraction and adapter replacement |

## Selection principles

- Start from required guarantees, not product popularity.
- Prefer no component over an unnecessary component.
- Prefer embedded or existing platform capabilities when their guarantees and blast radius are adequate.
- Do not add Redis, Kafka, a distributed database or a service mesh by habit.
- Do not remove durable infrastructure merely to reduce nominal cost when it would transfer cost into incidents or manual recovery.
- Keep idle edge footprint close to zero where possible; stores must not pay continuously for unused coordination mechanisms.
- Batch, compress and send deltas when they preserve latency and recovery requirements.
- Measure retry amplification and backlog recovery, not only happy-path throughput.
- Account for people and operational complexity as cost-to-serve.
- Revisit a component when unit cost grows faster than business volume or when a simpler adapter reaches the same guarantees.

## Required review output

An architecture review approving a component must record:

- plane membership;
- data classes affected;
- guarantees required;
- consistency and partition behavior;
- minimum topology;
- benchmark assumptions;
- cost allocation unit;
- simpler alternatives rejected;
- removal or replacement strategy;
- owner and review date.
