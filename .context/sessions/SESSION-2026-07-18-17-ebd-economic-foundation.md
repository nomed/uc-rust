# Session — Economics by Design economic foundation

- Date: 2026-07-18
- Repository: `nomed/uc-rust`
- Normative source: `nomed/ebd-bok`
- Status: Defined; implementation and evidence pending

## Objective

Adopt Economics by Design as a first-class architectural and governance concern in UC Rust, while preventing direct technical consumption from being confused with a meaningful Economic Unit or full Cost-to-Serve.

## Decisions recorded

- Accepted ADR-0019.
- Economics is a cross-cutting plane over Application, Data, Control and Coordination planes.
- `nomed/ebd-bok` is normative for adopted EbD concepts.
- Economic observations distinguish measured, attributed, allocated and estimated values.
- UC Rust follows Crawl, Walk and Run maturity.
- The first executable slice is the direct technical economics of one durable synchronized business event.
- Absolute euro gates remain deferred until representative workloads, rate cards and reproducible evidence exist.

## Artifacts created

- `.context/decisions/ADR-0019-economics-by-design-integration.md`
- `governance/economics/economic-units.yaml`
- `governance/economics/consumption-drivers.yaml`
- `governance/economics/costing-policy.yaml`
- `governance/economics/ebd-traceability.yaml`

## Initial economic unit

`uc.synced_business_event` is a candidate Economic Unit for the synchronization slice. It represents one accepted, durable, non-duplicate business event transferred from an authoritative source to its replica. Retries, duplicates and control messages are excluded from unit volume but remain visible as consumption and amplification.

It is not equivalent to a completed sale and is not a claim of full Cost-to-Serve.

## Next implementation slice

1. Add provider-neutral `uc-economics` types for identifiers, measurements, provenance and classifications.
2. Define a benchmark workload for durable SQLite enqueue, accept, duplicate and replay.
3. Instrument physical consumption without transaction payload or personal data.
4. Produce a machine-readable Economic Impact Report.
5. Compare the report against a controlled baseline using relative regression gates.
6. Update EbD traceability and reciprocal issues when implementation findings emerge.

## Maturity statement

The economic foundation is `Defined`. No claim is made that economic collection, attribution, benchmarks, CI gates or monetary costing are Implemented, Enforced, Evidenced or Accepted.
