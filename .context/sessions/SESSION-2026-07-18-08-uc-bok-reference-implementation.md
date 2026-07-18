# SESSION-2026-07-18-08 — UC-BoK reference implementation relationship

- Date: 2026-07-18
- Governing issues: nomed/uc-rust#38, nomed/uc-bok#9

## Objective

Make UC Rust the reference implementation of UC-BoK and establish a bidirectional, agent-operable feedback loop between normative knowledge and executable software.

## Completed

- Created reciprocal issues in UC-BoK and UC Rust.
- Accepted ADR-0014 in UC Rust.
- Accepted ADR-017 in UC-BoK.
- Added `governance/uc-bok-traceability.yaml` as the machine-readable mapping skeleton.
- Added UC-BoK specification alignment to the P0 system quality model.
- Updated Project Ready #19 to require traceability, drift validation and reciprocal feedback evidence.

## Decisions

- UC-BoK is normative for vendor-neutral Unified Commerce concepts, principles, capabilities, business objects, relationships, constraints and schemas.
- UC Rust is the reference implementation and executable validation laboratory.
- Implementation feedback is evidence submitted to UC-BoK governance, not an automatic normative change.
- UC-BoK changes require explicit UC Rust impact assessment.
- Intentional divergence requires accepted reciprocal decisions.

## Remaining implementation

- Populate the traceability manifest with the declared UC-BoK release/revision and initial mappings.
- Add CI validation and drift-report generation.
- Implement reciprocal issue creation/deduplication automation.
- Add issue #38 to the declarative GitHub governance manifest before the next governance apply.
- Extend agent instructions with mandatory UC-BoK impact and traceability assessment.
