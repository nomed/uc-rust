# Canonical Record Example Review

- Status: Complete
- Governing issue: #60
- Depends on: #56, #57, #58, #59

## Example set

The UC Rust 1.0 knowledge model is exercised by:

- `CR-0001` — Basket Capability;
- `CR-0002` — Pricing Capability;
- `RRR-0001` — Operation Invocation;
- `QAR-0001` — Operation Invocation Latency;
- `QAR-0002` — Operation Invocation Resilience;
- `QAR-0003` — Offline Operability;
- `ER-0001` — Operation Invocation Economic Unit.

## Trace demonstrated

```text
UC-BoK Basket/Pricing concepts
        ↓ realizes_concept
CR-0001 Basket        CR-0002 Pricing
        ↘              ↙
          RRR-0001 Operation Invocation
        ↑      ↑       ↑
   QAR-0001 QAR-0002 QAR-0003
        constrains runtime and capability behavior
                 ↑
              ER-0001
        constrains measurable cost attribution
```

The graph uses only controlled canonical directions. QAR and ER records own their constraints and point to the records they constrain. Inverse views are projection-only.

## Ambiguities resolved by the examples

1. Pricing is a Capability Record, not an Operation Record, rules-engine record or technology decision.
2. Operation Invocation is a reusable Runtime Responsibility Record, not the business meaning of Basket or Pricing.
3. Latency, resilience and offline operability remain separate measurable Quality Attribute Records rather than sections whose thresholds are duplicated across capabilities.
4. The economic unit is stable across providers and deployment profiles; provider prices remain measurement inputs.
5. A QAR or ER relation is a design constraint, not evidence that the target currently conforms.
6. Offline support is capability- and profile-specific; the quality record does not imply that every operation is available offline.
7. Numeric budgets may remain explicit acceptance placeholders during design, but they cannot be silently treated as satisfied or omitted at the release gate.

## Validator expectations

The canonical set must pass:

```bash
python scripts/validate_records.py docs/knowledge/records
python scripts/validate_record_graph.py docs/knowledge/records
```

The first command validates envelope, lifecycle, relation vocabulary and required body sections. The second validates duplicate identities, local resolution, authority namespaces, graph cycles and isolation.

## Runtime Foundation use

The records can drive Runtime Foundation design without committing to a crate framework, protocol, database, deployment topology or provider. They define:

- capability semantics and invariants;
- reusable invocation responsibility;
- measurable quality scenarios;
- economic attribution boundaries;
- central, edge and offline profile obligations;
- evidence that must be produced before acceptance and release disposition.

## Completion disposition

The example set is complete for the M0.5 gate. The records remain `Proposed` until their accountable reviewers accept their individual normative content. Completion of #60 proves the model and does not bulk-accept the examples.
