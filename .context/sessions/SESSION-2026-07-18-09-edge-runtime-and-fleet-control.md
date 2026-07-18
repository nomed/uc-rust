# SESSION-2026-07-18-09 — Edge runtime and fleet control

- Date: 2026-07-18
- Governing epic: #10
- Related issues: #19, #38, #39, #40, nomed/uc-bok#9, nomed/uc-bok#10

## Objective

Crystallize the retail requirement that selected Unified Commerce processes remain operational during WAN outage and that peripheral runtimes are securely deployed, inventoried and monitored from a central control plane.

## Decisions

- Accepted ADR-0015: UC Rust is a distributed central/edge system; edge is an autonomous runtime profile using the same domain and application operations as central.
- Accepted ADR-0016: UC Rust includes a desired-state/actual-state fleet control plane and a pull-based, signed, staged and rollback-capable edge update agent.
- Cache, central replicas and edge-authoritative data are separate concepts.
- Synchronization is a first-class capability with semantic authority, conflict and convergence rules.
- Technical health and retail-operational health are separate.
- Remote diagnostics use typed audited commands, not unrestricted permanent shell access.

## Work completed

- Created UC Rust issue #39 for offline capability profiles, synchronization and convergence.
- Created UC Rust issue #40 for deployment, inventory, monitoring, desired-state reconciliation and fleet security.
- Created reciprocal UC-BoK issue #10 to assess which edge and fleet principles belong in the normative Unified Commerce model.
- Added offline continuity, synchronization, fleet control and deployment safety to the P0 system quality model.
- Added #39 and #40 as hard dependencies of Project Ready #19.

## Evidence required next

- capability-by-capability offline classification;
- data-authority and conflict matrix;
- sync protocol and compatibility envelope;
- edge runtime composition model;
- ephemeral partition/reconciliation test environment;
- signed update manifest and simulated fleet rollout;
- inventory and retail-operational health schemas;
- UC-BoK disposition and stable identifiers.

## Remaining governance work

- Add issues #38–#40 to `governance/github-manifest.json`.
- Add explicit edge and fleet rules to `AGENTS.md`.
- Update Epic #10 child list and scope.

These remaining items are recorded explicitly and must be completed before context/governance alignment can be considered current.
