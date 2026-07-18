# ADR-0016 — Central control plane for edge fleet deployment and state

- Status: accepted
- Date: 2026-07-18
- Decision owners: Human project owner, Platform Architect
- Related issues: #28, #29, #32, #33, #39, #40, nomed/uc-bok#10

## Context

An offline-capable retail edge fleet cannot be managed reliably through ad-hoc remote access or one-shot push deployments. The central platform must know which edge nodes exist, which state they should have, which state they actually report and whether they remain retail-operational.

## Decision

UC Rust includes a central edge-fleet control plane and a local edge update/management agent.

The control plane owns desired state. Each edge owns installation safety and reports actual state. Reconciliation is pull-based and resilient to intermittent WAN connectivity.

Every edge has a unique device identity, tenant/location binding and independently revocable credentials. Fleet state includes hardware, OS, CPU architecture, deployed components, application release, database schema, synchronization protocol, authorization schema, configuration revision, certificates, capability profile, health and synchronization backlog.

Edge releases use signed immutable manifests and content-addressed artifacts. Before installation the edge verifies identity, signature, digest, provenance, compatibility, available resources, maintenance window, synchronization safety and absence of protected retail operations.

Updates are resumable and staged. The update process provides preflight, controlled quiescence, migration, atomic switch, health verification, commit and automatic rollback where safe. Application rollback must preserve business data; database changes follow forward-only expand/migrate/contract compatibility.

Rollouts are cohort-based and support laboratory, canary, pilot, regional, percentage and fleet-wide stages. Technical, synchronization and retail-operational metrics can pause or roll back a rollout.

Technical process health and retail-operational health are separate. A node may be `offline-operational` while disconnected, or technically running but unable to sell.

Remote management uses typed commands rather than permanent unrestricted shell access. Commands are authenticated, authorized, signed, expiring, idempotent, correlated and audited.

## Required guarantees

- WAN loss during download or installation cannot corrupt the active release.
- Unsigned, revoked, incompatible or unexpected artifacts and configurations are rejected locally.
- Desired/actual drift is visible and actionable.
- Telemetry and deployment results buffer locally while offline and upload later.
- Configuration has an independent, signed and schema-compatible lifecycle from application releases.
- Inventory and health queries support tenant, geography, hardware, release, drift and risk cohorts.
- Quarantine and certificate revocation stop a compromised edge from receiving secrets, sync data or releases.

## Consequences

- A compatibility envelope must cover central release, edge release, DB schema, sync protocol, contract version, configuration schema, authorization schema and UC-BoK revision.
- Fleet-management tests require ephemeral simulated edge nodes, WAN interruption, failed migrations, rollback, canary thresholds and drift scenarios.
- Monitoring must include business readiness signals such as sale capability, pricing snapshot validity, local database health and durable outbox safety.
- UC-BoK must determine which fleet and operational-profile concepts are normative through `nomed/uc-bok#10`.
