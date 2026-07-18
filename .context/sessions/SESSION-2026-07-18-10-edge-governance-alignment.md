# SESSION-2026-07-18-10 — Edge governance alignment

- Date: 2026-07-18
- Governing epic: #10
- Related issues: #19, #38, #39, #40
- Related decisions: ADR-0014, ADR-0015, ADR-0016

## Objective

Complete the declarative and agent-instruction alignment left open after accepting the UC-BoK reference-implementation, distributed retail runtime and edge fleet-control decisions.

## Actions completed

- Updated `AGENTS.md` with mandatory UC-BoK impact classification and traceability maintenance.
- Added central/edge deployment-profile rules and the requirement to reuse one canonical domain/application core.
- Added mandatory offline capability classifications.
- Added explicit separation of cache, replicated central data and edge-authoritative data.
- Added synchronization requirements for durable checkpoints, outbox/inbox, deduplication, ordering, reconciliation, bootstrap and re-sync.
- Added WAN-partition, restart, duplicate, delayed, reordered and semantic-conflict test requirements.
- Added fleet desired-state/actual-state, inventory, device identity, signed artifacts, compatibility-envelope, preflight, staged activation, canary, pause and rollback requirements.
- Added separate technical-health and retail-operational-health requirements.
- Added edge-specific performance, release-manifest and self-provisioned test-environment requirements.
- Updated `governance/github-manifest.json` so issues #38, #39 and #40 are managed under M0 and Project #4.

## Result

The previously declared administrative gaps are closed. The accepted decisions, issues, Project Ready gate, system quality model, agent instructions and declarative GitHub manifest now describe the same UC-BoK, edge-runtime and fleet-control architecture.

## Remaining work

Implementation and enforcement remain open under issues #38–#40 and gate #19. In particular, the project still needs the initial UC-BoK mapping, capability offline matrix, sync protocol design, fleet proof environment and executable validation workflows.
