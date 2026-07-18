# Handoff — UC Rust 1.0 planning

## Current focus

Complete architecture and delivery planning through 1.0 before implementing Runtime Foundation code.

## Authoritative planning artifacts

- `docs/roadmap/uc-rust-1.0-blueprint.md`
- `.context/decisions/ADR-0021-operation-first-architecture.md`
- `.context/rfcs/RFC-0001-runtime-foundation.md`
- `docs/architecture/runtime-foundation-review-checklist.md`
- `docs/roadmap/release-gates.md`

## Required next work

1. Create GitHub epics and child issues for the blueprint and Runtime Foundation.
2. Add M5–M8, RC and 1.0 milestones to the governed project model.
3. Map dependencies and parent/sub-issue relationships.
4. Review and accept the blueprint, ADR and RFC.
5. Only then begin the first runtime implementation issue.

## Important constraints

- Yukh is the future generic governance projection engine but does not block Stream B.
- Existing UC Rust governance remains write-authoritative during shadow migration.
- No generic runtime feature may be added without a concrete 1.0 requirement, quality budget and economic consequence.
