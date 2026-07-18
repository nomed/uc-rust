# Session — Governance invariants

- Session ID: SESSION-2026-07-18-02
- Date: 2026-07-18
- Governing epic: #10
- Related issues: #19, #21, #22, #23
- Status: Completed

## Objective

Crystallize three non-negotiable project requirements expressed by the human project owner:

1. `.context` must remain continuously current;
2. business logic and procedures must have one canonical implementation shared by REST, gRPC, workers, jobs and other adapters;
3. database migrations must be clean, deterministic and production-safe.

## Context loaded

- `.context/README.md`
- `.context/manifest.yaml`
- existing ADRs 0001–0003
- root `AGENTS.md`
- Epic #10 and Project Ready gate #19
- GitHub governance manifest

## Actions completed

- Created issue #21 for context freshness enforcement.
- Created issue #22 for the single application core architecture.
- Created issue #23 for database migration policy.
- Accepted ADR-0004 on mandatory context freshness.
- Accepted ADR-0005 on one canonical application core.
- Accepted ADR-0006 on forward-only expand/migrate/contract database migrations.
- Updated `AGENTS.md` with mandatory operational rules.
- Updated Epic #10 and gate #19.
- Registered issues #21–#23 in `governance/github-manifest.json`.

## Decisions crystallized

- Code and durable context must never knowingly diverge.
- A PR without required context updates or a reviewed no-impact justification is incomplete.
- Delivery adapters are translation and invocation boundaries, not owners of business procedures.
- Domain behavior and application operations are the only canonical locations for business logic and orchestration.
- Applied migrations are immutable and production evolution is forward-only.
- Breaking schema changes use expand/migrate/contract and must support rolling deployments.

## Evidence

- `.context/decisions/ADR-0004-mandatory-context-freshness.md`
- `.context/decisions/ADR-0005-single-application-core.md`
- `.context/decisions/ADR-0006-forward-only-database-migrations.md`
- `AGENTS.md`
- `governance/github-manifest.json`

## Remaining work

- Implement context-impact checks and record validation in CI under #21.
- Define crate boundaries and architecture tests under #22 and #12.
- Select and validate the concrete migration runner under #23.
- Add pull-request and issue templates under #17.
- Add these controls as required evidence for Project Ready gate #19.

## Handoff

No immediate handoff is required. The next planning session should incorporate ADR-0004, ADR-0005 and ADR-0006 into the target architecture, agent roles and delivery workflow.
