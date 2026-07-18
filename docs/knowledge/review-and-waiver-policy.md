# Review, Freshness and Waiver Policy

- Status: Complete
- Governing issue: #57
- Governing decision: ADR-0023
- Related: ADR-0022, `docs/architecture/record-lifecycle.md`

## Purpose

This policy defines how accepted architecture knowledge remains reviewable and safely usable over time without conflating normative lifecycle, implementation conformance, verification freshness or release inclusion.

Freshness is a derived projection. It never changes a record lifecycle state automatically.

## Review classes

Every Accepted or Deprecated record declares one review class:

- **continuous** — event-triggered review for security, fiscal, public-contract and high-risk operational records;
- **release** — reviewed at every affected release gate;
- **periodic** — reviewed at a declared maximum interval;
- **stable** — reviewed only on explicit triggers, suitable for foundational principles with low expected volatility.

A record may combine a periodic interval with event triggers. `stable` never means exempt from review: it requires explicit triggers and an accountable owner.

## Minimum review metadata

The `review` block contains:

- `required_roles` — roles required to disposition the record;
- `reviewers` — attributable reviewers participating in the current or latest review;
- `disposition` — `pending`, `accepted`, `rejected` or `withdrawn`;
- `review_class` — `continuous`, `release`, `periodic` or `stable`;
- `reviewed_at` — date of the latest completed review;
- `next_review_at` — explicit next due date when date-based review applies;
- `review_interval_days` — maximum interval when periodic calculation is used;
- `event_triggers` — named events requiring review;
- `overdue_grace_days` — bounded period during which reliance may remain allowed after review becomes due;
- `escalation` — accountable role or issue route when review becomes overdue;
- `unresolved_objections` — objections that remain open.

For an Accepted record, `reviewed_at` and `review_class` are mandatory. A periodic record must declare either `next_review_at` or `review_interval_days`. A continuous or stable record must declare at least one event trigger.

## Review triggers

The initial controlled trigger set is:

- `incident`;
- `security_finding`;
- `privacy_finding`;
- `fiscal_or_legal_change`;
- `external_standard_change`;
- `public_contract_break`;
- `quality_budget_failure`;
- `economic_threshold_breach`;
- `dependency_or_provider_change`;
- `runtime_profile_change`;
- `release_gate`;
- `ownership_loss`;
- `manual_request`.

Type-specific policies may narrow the trigger set but may not disable incident, security, legal or ownership-loss review where relevant.

## Freshness calculation

Freshness is calculated deterministically from authoritative review metadata and detected events:

- `current` — no due date or trigger has been breached;
- `review_due` — a review is required, but bounded continued reliance is allowed during the grace period;
- `stale` — safe reliance is no longer established;
- `unknown` — required review metadata is missing or cannot be evaluated.

Evaluation order:

1. missing accountable owner or required review metadata produces `unknown`, except ownership loss on a previously valid record produces `review_due` immediately;
2. an unreviewed matching event trigger produces `review_due`;
3. a reached due date produces `review_due`;
4. expiration of `overdue_grace_days` produces `stale`;
5. absent breaches produce `current`.

High-risk SR, IR, DR and QAR records default to zero grace and therefore become `stale` when overdue unless a valid waiver explicitly permits bounded reliance. Other families default to `review_due`; type-specific policy may tighten this.

Freshness changes do not modify `status`, `content_version` or accepted history.

## Review outcomes

A review produces exactly one append-only outcome:

- `confirmed_without_change`;
- `confirmed_with_editorial_update`;
- `semantic_revision_required`;
- `deprecate`;
- `supersede`;
- `revoke_waiver`;
- `escalate_unresolved_risk`.

Every review event records actor, date, reviewed content version, evidence, findings, outcome and follow-up issue where required.

`semantic_revision_required` reopens the record through the lifecycle rules in `docs/architecture/record-lifecycle.md`; it does not mutate Accepted meaning silently.

## Waivers

A waiver is not a lifecycle state and does not modify the normative record. It is a governed, temporary exception attached to a conformance, freshness or release-gate finding.

Every waiver contains:

- stable identifier;
- affected record;
- affected relation, release, implementation or environment scope where applicable;
- deviation description and rationale;
- accountable owner and human approver;
- residual risk;
- compensating controls;
- creation date;
- mandatory expiry date;
- remediation issue;
- evidence references;
- disposition: `active`, `expired`, `revoked` or `remediated`.

Permanent waivers and self-approved waivers are forbidden. A waiver cannot accept, deprecate or supersede a record and cannot conceal a failed validation result.

Expiry is evaluated automatically. An expired or revoked waiver becomes invalid and reopens the underlying finding. Remediation closes the finding only when the required evidence is recorded.

## Supersession and deprecation review

A review recommending deprecation or supersession must identify:

- affected scope;
- compatibility and migration impact;
- successor records where applicable;
- records and releases requiring impact review;
- required lifecycle transition authority.

A review recommendation alone does not perform the lifecycle transition.

## Ownership loss

A record with no accountable owner becomes `review_due` immediately. If ownership is not restored within the configured grace period, the record becomes `stale` and cannot satisfy a release-readiness gate.

Ownership reassignment is an attributable governance event and does not change the record's normative meaning.

## Incident and regression handling

An incident, security finding, contract break, cost regression or failed quality budget must identify potentially affected records. Yukh may calculate a candidate impact set from typed relations, but owners decide the review outcome and any normative lifecycle transition.

An unresolved high-severity trigger blocks affected release gates unless an authorized, unexpired waiver exists.

## Machine-detectable invariants

Validation and projections must detect:

1. Accepted records missing review class or last review date;
2. periodic records missing a due date or interval;
3. continuous/stable records missing event triggers;
4. next-review dates earlier than the last review;
5. overdue records and expired grace periods;
6. ownerless records;
7. permanent, expired, self-approved or unscoped waivers;
8. waivers missing remediation issues, risk or compensating controls;
9. release readiness relying on stale records without a valid waiver;
10. review outcomes that imply a lifecycle transition but lack the required transition event.

## Completion statement

This policy completes issue #57 together with the normative lifecycle state machine and ADR-0023. It preserves accepted history while making stale knowledge, review obligations, temporary exceptions and supersession decisions auditable and machine-detectable.