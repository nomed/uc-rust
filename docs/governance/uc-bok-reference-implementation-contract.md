# UC-BoK ↔ UC Rust Reference Implementation Contract

- Status: Reviewable for acceptance
- Version: 1.0
- Governing issue: #38
- Governing decision: ADR-0030
- Normative peer: `nomed/uc-bok/specifications/018-uc-rust-integration/operating-contract.md`

## Purpose

`nomed/uc-bok` is the normative Unified Commerce specification repository. `nomed/uc-rust` is its reference implementation and executable feedback environment.

Every traceability baseline pins an immutable UC-BoK revision and the corresponding generated identifier manifest. UC Rust does not claim conformance to an unpinned moving branch.

## Responsibilities

UC-BoK publishes normative concepts, schemas and typed stable identifiers. UC Rust maps those identifiers to domain types, canonical Operations, public contracts, migrations, fixtures and tests.

Implementation evidence may challenge the normative model through reciprocal issues, but does not silently redefine it.

## Typed identifiers

Mappings use `(ucbok_type, ucbok_id)` rather than an untyped string. Example keys are `capability:basket-management`, `business-object:basket` and `capability:checkout`.

## Status vocabulary

- `implemented`: verified implementation and qualifying evidence exist;
- `partial`: a bounded subset is implemented and the missing scope is explicit;
- `planned`: accepted roadmap ownership exists without implementation evidence;
- `not-applicable`: outside the declared reference scope, with rationale and approval;
- `intentionally-divergent`: reciprocal accepted decisions document the divergence.

A normative identifier absent from the local catalog is reported as `unmapped`; it is never assumed implemented or not applicable.

## Mapping evidence

Each local mapping records the pinned normative revision, owner, implementation paths, Operation/contract references, evidence paths, target gate and compatibility notes. Documentation alone cannot prove an executable `implemented` claim.

## Normative change workflow

1. Pin the new UC-BoK revision and manifest.
2. Diff typed identifiers and affected normative content.
3. Open or update a fingerprinted adoption-impact issue.
4. Record compatibility, migration and release-gate impact.
5. Dispose as `adopted`, `partial-adoption` or `intentional-divergence`.
6. Update the baseline only after attributable reciprocal disposition.

## Implementation feedback workflow

1. Capture reproducible implementation evidence.
2. Identify affected typed UC-BoK identifiers.
3. Derive the stable fingerprint from source repository, source reference and sorted typed identifiers.
4. Reuse an existing open reciprocal issue with the same fingerprint or create linked issues.
5. UC-BoK records `accepted`, `accepted-with-modification`, `rejected` or `deferred`.
6. UC Rust feedback is not resolved until that normative disposition is linked.

## Drift report

The generated report shows pinned revision, inventory totals, mapped/unmapped counts, status distribution, P0 gaps, stale identifiers, broken repository references, unsupported implementation claims and unresolved reciprocal decisions.

Aggregate coverage does not override capability-specific release gates.

## Validation policy

Validation rejects unknown typed identifiers, invalid statuses, duplicate keys/fingerprints, missing required fields, broken local paths, unsupported `implemented` claims and divergences without reciprocal accepted decisions.

The executable automation remains an M1 obligation and must not be claimed before it exists.

## Change control

Material changes to authority, status semantics, fingerprinting, closure rules or divergence governance require accepted reciprocal decisions and a version bump in both repositories.