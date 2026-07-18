---
id: uc-rust:CR-0002
type: CR
schema_version: 1
content_version: 0.1.0
title: Pricing Capability
summary: Implementation-independent pricing capability for deterministic retail price, promotion, tax and adjustment outcomes.
status: Proposed
owners:
  - role: domain-architecture
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Pricing semantics and invariants used to calculate authoritative basket and line outcomes across runtime profiles.
non_goals:
  - Concrete rules-engine technology or deployment topology
  - Ownership of product, customer, tax-master or promotion-source data
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/60
relations:
  - type: realizes_concept
    target: uc-bok:CAP-COMMERCE-PRICING
  - type: depends_on
    target: uc-rust:RRR-0001
    scope: governed operation invocation and deterministic disposition
review:
  required_roles: [domain-architecture, product-analysis, economics]
  reviewers: []
  disposition: pending
  review_class: release
  event_triggers: [fiscal_or_legal_change, quality_budget_failure, economic_threshold_breach, dependency_or_provider_change]
---

# CR-0002 — Pricing Capability

## Purpose

Own the business meaning of retail pricing outcomes independently of the rules engine, storage model, transport, deployment profile or vendor implementation.

## Actors and operations

Authorized channels and operators request pricing for a basket, line, customer context, fulfillment context or correction. Operations include quote, recalculate, explain, validate, reserve a bounded outcome and finalize the pricing basis used by a commercial transaction.

## Outcomes and events

Pricing produces item prices, taxes, discounts, surcharges, totals, eligibility findings, applied-rule references and an explanation trace suitable for audit. Relevant events include pricing requested, context changed, rule set selected, outcome calculated, outcome rejected, outcome expired and pricing finalized.

## Invariants

1. The same governed inputs, rule-set identity and execution profile produce the same semantic outcome.
2. Every monetary outcome identifies currency, precision, rounding policy and authoritative rule or policy revision.
3. A pricing result distinguishes calculated, estimated, reserved, finalized and invalidated outcomes.
4. Promotion eligibility and application are explainable without exposing protected implementation details.
5. Offline pricing declares permitted rule and data staleness and never silently weakens fiscal, legal or security constraints.
6. Recalculation does not duplicate an already-applied adjustment when invocation identity and pricing context are unchanged.
7. Conflicting authorities are surfaced as an explicit failure or bounded precedence decision, never resolved by incidental processing order.
8. Finalized transaction pricing is immutable except through an explicit correction or reversal capability.

## Authority

This record owns pricing semantics and outcome invariants. Product, customer, tax, promotion-source and inventory authorities remain external. Runtime invocation mechanics belong to RRR-0001; measurable latency, resilience, offline and economic constraints belong to QAR and ER records.

## Runtime profiles

Central, store-edge and offline-capable profiles may use different realizations. Each profile declares available pricing operations, local rule/data inventory, freshness limits, fallback policy, reconciliation behavior and operations that must be blocked.

## Failure model

Unsupported context, unavailable authority, expired rule set, incompatible currency or jurisdiction, ambiguous precedence, invalid input, deadline exceeded and indeterminate execution are explicit dispositions with stable error semantics.

## Evidence plan

Acceptance requires envelope and graph validation, deterministic pricing fixtures, rounding and tax boundary scenarios, promotion explanation examples, stale-rule behavior, offline reconciliation, and traceability to latency, resilience, offline-operability and economic records.
