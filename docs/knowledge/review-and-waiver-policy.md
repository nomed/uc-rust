# Review, Freshness and Waiver Policy

- Status: Proposed
- Governing issue: #57
- Related: ADR-0023

## Review classes

Records declare one review class:

- **continuous** — event-triggered review for security, fiscal, public contract and high-risk operational records;
- **release** — reviewed at every affected release gate;
- **periodic** — reviewed at a declared maximum interval;
- **stable** — reviewed only on explicit triggers, suitable for foundational principles with low expected volatility.

A record may combine periodic and event-driven review.

## Minimum review metadata

- accountable owner;
- required reviewer roles;
- review class;
- last completed review;
- next due date or interval;
- event triggers;
- escalation policy when overdue.

## Freshness calculation

Freshness is a projection from review metadata and detected events:

- `current` — no due date or trigger has been breached;
- `review_due` — review is due but safe use remains explicitly allowed;
- `stale` — safe reliance is no longer established;
- `unknown` — insufficient metadata exists.

High-risk SR, IR, DR and QAR records default to `stale` when overdue unless a waiver exists. Other families default to `review_due`; type-specific policy may tighten this.

## Waivers

A waiver is not a lifecycle state and does not modify the normative record. It is a governed exception attached to a conformance or gate finding.

Every waiver contains:

- stable identifier;
- affected record, relation, release and implementation scope;
- deviation description and rationale;
- owner and approver;
- residual risk;
- compensating controls;
- creation and expiry dates or an explicit review trigger;
- remediation issue;
- evidence references.

Permanent waivers are forbidden. Expired waivers become invalid automatically and re-open the underlying finding.

## Review outcomes

A review produces one of:

- confirmed without change;
- confirmed with editorial update;
- semantic revision required;
- deprecate;
- supersede;
- revoke waiver;
- escalate unresolved risk.

Each outcome is recorded as an append-only review event.

## Ownership loss

A record with no accountable owner becomes `review_due` immediately. If no owner is restored within the configured grace period, it becomes `stale` and cannot satisfy release readiness.

## Incident and regression triggers

An incident, security finding, contract break, cost regression or failed quality budget must identify potentially affected records. Yukh may calculate the impact set from typed relations, but owners decide the resulting normative transition.
