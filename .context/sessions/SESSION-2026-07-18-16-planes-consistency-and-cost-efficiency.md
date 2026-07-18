# Session 2026-07-18-16 — Planes, consistency and cost efficiency

## Governing issues

- `nomed/uc-rust#29`
- `nomed/uc-rust#31`
- `nomed/uc-rust#37`
- `nomed/uc-rust#39`
- `nomed/uc-rust#40`
- `nomed/uc-rust#42`

## Objective

Prevent edge deployment, business-data consistency and distributed coordination from becoming one ambiguous mechanism, while making cost-to-serve a measurable architectural constraint.

## Decisions

- accepted ADR-0018
- separated application, data, control and coordination planes
- declared deployment desired/actual state independent from business-data replication
- prohibited one universal consistency model
- required a complete consistency declaration for each synchronized data class
- made consensus an explicit reviewed exception
- required significant components to document minimum topology, footprint, scale unit, operational burden and simpler alternatives
- adopted the least costly solution that satisfies measured guarantees as the selection principle

## Governance changes

- added `docs/architecture/planes-consistency-and-cost-to-serve.md`
- added `Cost efficiency` as the twenty-fifth P0 quality attribute
- expanded Performance and Consistency thresholds
- updated repository validation to require ADR-0018, the architecture policy, the component scorecard and the consensus decision test
- corrected the stale AGENTS heading expected by the validator

## UC-BoK impact

- classification: `clarification-required`
- rationale: the distinction between application, data, control and coordination planes and the per-data-class consistency declaration are architectural concepts that should be evaluated for normative inclusion in UC-BoK
- no implementation divergence is introduced in this session

## Maturity

- architectural policy: Defined
- cost-efficiency quality attribute: Defined
- repository governance validation: Implemented
- component benchmark evidence: not yet produced
- Project Ready acceptance: not yet requested

## Next work

1. add a machine-readable synchronized data-class manifest and schema
2. declare the first Basket/sale business-effect data class
3. define explicit cost and resource budgets for the initial central and store-edge profiles
4. review every proposed runtime component against the scorecard before adoption
5. resume the central/store-edge harness only after its data-plane and control-plane responsibilities are explicitly separated
