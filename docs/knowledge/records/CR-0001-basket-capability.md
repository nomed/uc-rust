---
id: uc-rust:CR-0001
type: CR
schema_version: 1
content_version: 0.2.0
title: Basket Capability
summary: Implementation-independent basket capability across channels and runtime profiles.
status: Proposed
owners: [{role: domain-architecture}]
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Basket business semantics and invariants from creation to finalization.
non_goals: [Runtime transport/protocol/retry mechanics, Concrete storage schema or service decomposition]
provenance:
  - {kind: issue, locator: https://github.com/nomed/uc-rust/issues/56}
  - {kind: uc-bok, locator: uc-bok:CAP-COMMERCE-BASKET}
relations:
  - {type: realizes_concept, target: uc-bok:CAP-COMMERCE-BASKET}
  - {type: depends_on, target: uc-rust:RRR-0001, scope: operation invocation mechanics}
review:
  required_roles: [domain-architecture, product-analysis]
  reviewers: []
  disposition: pending
---

# CR-0001 — Basket Capability

## Purpose
Own the business semantics of a retail basket while delegating execution mechanics to runtime responsibility records.

## Operations
Create; add/change/remove lines; identify customer; apply pricing/promotions; attach fulfillment intent; validate; suspend/resume/abandon/expire; finalize.

## Events
Basket created; line changed; customer context changed; pricing recalculated; restriction changed; basket suspended/resumed/abandoned/expired; validated; finalized.

## Invariants
1. Totals derive from authoritative lines, prices, adjustments and taxes.
2. Finalized baskets are immutable except through explicit correction.
3. Every mutation has actor and causation context.
4. Idempotent commands cannot apply an effect twice.
5. Pricing outcomes identify rule/authority version.
6. Offline operation cannot silently weaken legal, fiscal or security controls.
7. Concurrent mutations use an explicit consistency policy.
8. Expiration/abandonment differ from finalization.

## Authority
Owns basket semantics, not product, customer, inventory, payment, fiscal-document or fulfillment authority.

## Offline and consistency
Profiles declare available operations, permitted data/rule staleness, durability, reconciliation, conflict policy and blocked operations. Strong consistency is required where duplicate finalization or contradictory ownership could occur.

## Security, economics and quality
Authorize by actor/channel/tenant; minimize customer data; audit overrides; validate untrusted totals. Evidence must support cost and quality budgets for mutation, recovery, availability, duplicates, reconciliation and audit completeness. Thresholds belong to ER/QAR records.

## Acceptance evidence
Envelope/schema validation; UC-BoK traceability; at least one RRR implementation relation; invariant tests; one offline reconciliation scenario; inclusion or explicit deferral in a Release Record.

## ADR-0022 validation
Confirms that capability semantics, runtime mechanics and evidence can be separated without standalone Operation, Event, Policy or Deployment record families.
