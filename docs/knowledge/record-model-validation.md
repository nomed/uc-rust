# Architecture Knowledge Record Model — Canonical Validation

- Status: Draft
- Governing issue: #56
- Governing decision: ADR-0022
- Canonical examples: CR-0001, RRR-0001

## Result

The examples validate the separation proposed by ADR-0022:

- `CR-0001` owns implementation-independent Basket semantics;
- `RRR-0001` owns reusable operation-invocation runtime semantics;
- operations/events remain governed concepts rather than standalone record families;
- code, tests, traces, reports and deployments remain evidence;
- runtime profiles do not justify Deployment Records;
- technology choice remains an ADR supported by evaluation evidence.

## Envelope findings

1. Accountable roles may be used before named owners/reviewers are assigned.
2. Relations require a controlled vocabulary finalized by the relationship-model issue.
3. `status` means knowledge lifecycle, not implementation progress.
4. Canonical examples use semantic `content_version`.
5. Markdown with YAML front matter is the canonical source representation.
6. Family-specific bodies remain Markdown until schemas define their structured projection.

## Taxonomy dispositions

| Family | 1.0 disposition |
|---|---|
| ADR, RFC, CR, RRR, QAR, ER, SR, IR, DR | Retain. |
| TR | Defer: two concrete normative 1.0 uses are not demonstrated; evaluation remains ADR evidence. |
| RR | Retain narrowly for normative release composition, gates, compatibility and exceptions. |

## Remaining blockers before ADR acceptance

- lifecycle model accepted;
- relationship vocabulary accepted;
- schemas for envelope, CR and RRR;
- migration path for existing ADR/RFC files;
- identifier-resolution boundary with UC-BoK, Yukh and EbD;
- accountable human review of CR-0001 and RRR-0001.

## Implementation readiness

Issue #56 now has canonical validation evidence, but UC Rust 1.0 implementation remains blocked until the planning epic resolves lifecycle, relationships, schemas, boundaries and canonical-record review.
