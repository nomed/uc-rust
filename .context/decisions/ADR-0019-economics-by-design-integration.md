# ADR-0019 — Economics by Design integration

- Status: Accepted
- Date: 2026-07-18
- Governing repositories: `nomed/uc-rust`, `nomed/ebd-bok`
- Related records: ADR-0012, ADR-0015, ADR-0018

## Context

UC Rust already treats performance and cost-to-serve as architectural concerns. Aggregate infrastructure cost alone is insufficient: architecture must explain how business demand produces technical consumption, attributable cost and ultimately Cost-to-Serve for economically meaningful units.

`nomed/ebd-bok` defines Economics by Design (EbD): software products are designed with economic consequences as first-class architectural requirements. Its Unit Economics Chain proceeds from business value and capability/offering through Economic Unit and consumption drivers to technical architecture, operating cost, Cost-to-Serve, margin and strategic decisions.

A technically precise cost per request or event is not automatically a meaningful economic unit and must not be presented as full Cost-to-Serve without an explicit costing policy.

## Decision

1. `nomed/ebd-bok` is normative for Economics by Design concepts adopted by UC Rust. UC Rust is an executable reference implementation and evidence environment for selected EbD concepts.
2. Economics is a cross-cutting plane. It observes and governs the Application, Data, Control and Coordination planes without owning their business or coordination semantics.
3. Every significant capability and architecture component must identify:
   - business value and offering boundary where known;
   - candidate and meaningful Economic Units;
   - consumption drivers;
   - direct, shared and human cost components;
   - costing and allocation policies;
   - expected scale curve and cost behavior;
   - economic owner, evidence and maturity.
4. Economic Units and technical consumption units are distinct. API calls, CPU time, bytes and database operations are consumption drivers unless explicitly accepted as economically meaningful units.
5. Cost observations are classified as:
   - `measured`: directly observed resource consumption or billed cost;
   - `attributed`: causally assigned through a declared driver;
   - `allocated`: shared cost distributed through a declared policy;
   - `estimated`: modelled before sufficient observation exists.
6. Every economic amount records currency, period, rate-card/cost-source revision, costing-policy revision, confidence and provenance.
7. UC Rust adopts the EbD maturity path incrementally:
   - Crawl: direct technical cost;
   - Walk: blended infrastructure, SaaS, licence and shared-service cost;
   - Run: full Cost-to-Serve including support, operations and attributable human capital.
8. The initial executable slice measures the direct technical cost of one durable synchronized business event. It must not be labelled full Cost-to-Serve.
9. Economic regression gates begin with stable physical consumption metrics and relative regressions. Absolute monetary gates become enforceable only when rate cards and costing policies are sufficiently reliable.
10. Economic observability must not create material cost or latency amplification. Its own overhead is measured and budgeted.
11. No vendor-specific FinOps system may leak into domain or application contracts. Collection, billing import and reporting systems remain replaceable adapters.
12. An architectural change is incomplete when its material economic impact is unknown, unmeasured without justification, or disconnected from an Economic Unit.

## Initial governed artifacts

- `governance/economics/economic-units.yaml`
- `governance/economics/consumption-drivers.yaml`
- `governance/economics/costing-policy.yaml`
- `governance/economics/ebd-traceability.yaml`

## Consequences

- Architecture reviews include economic causality, not only infrastructure price.
- Cost per transaction becomes available where transaction is an accepted Economic Unit, while other units such as store/day, terminal/month and tenant/month remain independently modelled.
- Comparisons between SQLite, brokers, quorum systems and other components include guarantees, performance, operational burden and unit-cost consequences.
- Early economic values remain explicitly provisional; precision and completeness mature separately.
- UC Rust can generate economic impact evidence without claiming that current repository maturity is Evidenced or Accepted.
