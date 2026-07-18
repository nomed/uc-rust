---
id: uc-rust:CR-0002
type: CR
schema_version: 1
content_version: 0.2.0
title: Pricing Capability
summary: Implementation-independent pricing capability for deterministic retail price, promotion, tax and adjustment outcomes.
status: Proposed
owners:
  - role: domain-architecture
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Pricing semantics and invariants used to calculate authoritative basket and line outcomes across runtime profiles and governed realizations.
non_goals:
  - Concrete rules-engine technology or deployment topology
  - Ownership of product, customer, tax-master or promotion-source data
  - Mandating a native pricing engine or a specific external provider
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/60
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/68
relations:
  - type: realizes_concept
    target: uc-bok:CAP-COMMERCE-PRICING
  - type: depends_on
    target: uc-rust:RRR-0001
    scope: governed Operation invocation and deterministic disposition
  - type: governed_by
    target: ADR-0024
    scope: native, delegated, composed, pipeline and hybrid realizations
review:
  required_roles: [domain-architecture, product-analysis, economics]
  reviewers: []
  disposition: pending
  review_class: release
  event_triggers: [fiscal_or_legal_change, quality_budget_failure, economic_threshold_breach, dependency_or_provider_change]
---

# CR-0002 — Pricing Capability

## Purpose

Own the business meaning of retail pricing outcomes independently of the rules engine, storage model, transport, deployment profile, provider or realization form.

UC Rust owns the canonical pricing semantics. Execution may be provided by a native implementation, a delegated system such as RGK, a composition, a governed pipeline or a hybrid realization selected by runtime policy.

## Actors and operations

Authorized channels and operators request pricing for a basket, line, customer context, fulfillment context or correction. Operations include quote, recalculate, explain, validate, reserve a bounded outcome and finalize the pricing basis used by a commercial transaction.

Consumers invoke canonical pricing Operations and never provider-specific methods.

## Outcomes and events

Pricing produces item prices, taxes, discounts, surcharges, totals, eligibility findings, applied-rule references and an explanation trace suitable for audit. Relevant events include pricing requested, realization selected, context changed, rule set selected, outcome calculated, outcome rejected, outcome expired and pricing finalized.

## Invariants

1. The same governed inputs, rule-set identity, semantic contract and execution profile produce the same semantic outcome within declared provider capabilities.
2. Every monetary outcome identifies currency, precision, rounding policy and authoritative rule or policy revision.
3. A pricing result distinguishes calculated, estimated, reserved, finalized and invalidated outcomes.
4. Promotion eligibility and application are explainable without exposing protected implementation details.
5. Offline pricing declares permitted rule and data staleness and never silently weakens fiscal, legal or security constraints.
6. Recalculation does not duplicate an already-applied adjustment when invocation identity and pricing context are unchanged.
7. Conflicting authorities are surfaced as an explicit failure or bounded precedence decision, never resolved by incidental processing order.
8. Finalized transaction pricing is immutable except through an explicit correction or reversal capability.
9. Changing realization does not change canonical input, output, error or outcome semantics silently.
10. Provider-specific payloads and identifiers do not leak into the canonical pricing contract except as governed evidence references.

## Authority

This record owns pricing semantics and outcome invariants. Product, customer, tax, promotion-source and inventory authorities remain external. A selected realization may own calculation execution and may be authoritative for its declared outcome, but this authority must be explicit in its realization manifest.

Runtime invocation mechanics belong to RRR-0001. Realization binding belongs to ADR-0024 and #68. Measurable latency, resilience, offline and economic constraints belong to QAR and ER records.

## Realizations

Supported realization forms are:

- native UC Rust pricing;
- delegated external pricing provider;
- composed pricing from multiple governed providers;
- pipeline pricing with explicit semantic stages;
- hybrid central/edge realization.

RGK is an initial delegated proving case, not a mandatory dependency and not the definition of the capability.

Every realization must pass shared semantic conformance fixtures and declare supported Operations, semantic versions, authority, limits, offline behavior, failure guarantees, compatibility and economic attribution.

## Runtime profiles

Central, store-edge and offline-capable profiles may use different realizations. Each profile declares available pricing Operations, selected or eligible realizations, local rule/data inventory, freshness limits, fallback policy, reconciliation behavior and Operations that must be blocked.

## Failure model

Unsupported context, unavailable realization, expired rule set, incompatible semantic/provider version, incompatible currency or jurisdiction, ambiguous precedence, invalid input, deadline exceeded and indeterminate execution are explicit dispositions with stable canonical error semantics.

Fallback between realizations is permitted only when authority, legal, fiscal, security and semantic guarantees remain valid and the decision is observable.

## Evidence plan

Acceptance requires envelope and graph validation, deterministic pricing fixtures, rounding and tax boundary scenarios, promotion explanation examples, stale-rule behavior, offline reconciliation, native/delegated conformance equivalence, provider failure and timeout behavior, realization-selection evidence, and traceability to latency, resilience, offline-operability and economic records.
