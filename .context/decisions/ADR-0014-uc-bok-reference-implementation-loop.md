# ADR-0014 — UC Rust as the reference implementation of UC-BoK

- Status: accepted
- Date: 2026-07-18
- Governing issues: nomed/uc-rust#38, nomed/uc-bok#9

## Context

UC-BoK defines a vendor-neutral, model-driven Unified Commerce body of knowledge through ontology, schemas, canonical catalogs and a queryable graph. UC Rust is intended to turn those concepts into executable software. Without a formal relationship, the repositories can silently diverge: implementation may invent semantics not represented in UC-BoK, while new normative material may never be assessed by UC Rust.

## Decision

`nomed/uc-bok` is the normative specification and knowledge repository. `nomed/uc-rust` is its reference implementation and executable validation laboratory.

The relationship is bidirectional:

- UC Rust artifacts map to stable UC-BoK identifiers and a declared UC-BoK revision.
- UC Rust raises structured implementation feedback when it finds ambiguity, contradiction, missing concepts, unimplementable constraints or useful extensions.
- UC-BoK changes raise structured adoption and impact-assessment work in UC Rust.
- Neither repository silently overrides the other.
- Intentional divergence requires accepted reciprocal decision records, explicit rationale and compatibility consequences.

## Traceability

A machine-readable traceability manifest will record:

- UC-BoK repository revision or release;
- UC-BoK concept/schema identifiers;
- UC Rust crates, modules, types, operations, contracts, migrations and tests;
- implementation status;
- evidence and governing issues;
- divergence disposition when applicable.

Allowed status values are `implemented`, `partial`, `planned`, `not-applicable` and `intentionally-divergent`.

## Cross-repository feedback

Every cross-repository issue includes source repository and evidence, affected identifiers/artifacts, change type, normative and implementation impact, reproduction/example, compatibility considerations, acceptance criteria and reciprocal issue link. Repeated findings are deduplicated through a stable fingerprint.

## Consequences

- UC-BoK alignment becomes a P0 quality attribute and Project Ready gate condition.
- Domain and contract work cannot be considered complete without traceability assessment.
- Agents may open issues in either repository under the cross-repository issue contract.
- UC-BoK remains vendor-neutral; implementation-specific choices stay in UC Rust unless they reveal a general principle worth promoting.
- The reference implementation may expose defects in the specification; implementation feedback is evidence, not an automatic normative change.
