# UC Rust Project Charter

- Status: Accepted
- Governing issue: #11
- Normative knowledge source: `nomed/uc-bok`
- Reference implementation: `nomed/uc-rust`
- Last updated: 2026-07-18

## Mission

Build UC Rust as a high-integrity, high-performance reference implementation of the Unified Commerce concepts defined by UC-BoK, suitable for retail environments that require central orchestration, multiple touchpoints, edge autonomy during WAN disruption and governed convergence of distributed state.

## Problem statement

Retailers need commerce capabilities that work coherently across stores, mobile devices, self-service touchpoints, digital channels, central services and peripheral edge runtimes. Existing systems frequently duplicate business procedures across POS, APIs, workers and local/offline implementations; couple business behavior to databases or vendors; and provide weak traceability between domain meaning, implementation, deployment and operational evidence.

UC Rust addresses this by providing one canonical application core, replaceable infrastructure adapters, explicit distributed/offline guarantees, executable contracts, complete quality enforcement and traceability to UC-BoK.

## Strategic rationale

UC-BoK defines the language, concepts, capabilities, business objects, relationships and normative constraints of Unified Commerce. UC Rust provides executable proof that these models can be implemented coherently. The implementation feeds ambiguity, contradictions and missing concepts back to UC-BoK, while new UC-BoK material is assessed explicitly before adoption.

Market evidence reinforces the need for modular POS/touchpoints, API-first and headless delivery, mobility, OMS integration, smart checkout/IoT extension, edge resilience, AI-assisted workflows and strong disaster recovery. UC Rust treats these as evidence to prioritize and validate architecture, not as an instruction to implement every market feature immediately.

## Target segment

The 1.0 reference implementation targets multi-location retailers that require a coherent basket-to-order journey across central and store-edge execution. It is especially relevant where stores must continue selected operations during WAN degradation and where platform teams need replaceable integrations, fleet governance and measurable cost-to-serve.

The project is not limited to a grocery, fashion or hospitality implementation. Sector-specific fiscal, payment, device and workflow differences enter through explicit capability and provider boundaries rather than becoming implicit core semantics.

## Target users and stakeholders

- Retail platform architects and engineering teams.
- Product and domain teams designing Unified Commerce capabilities.
- Store operations and support teams requiring resilient edge execution.
- Integration teams connecting ERP, CRM, OMS, payment, inventory, loyalty and device ecosystems.
- Security, reliability and operations teams governing distributed retail fleets.
- AI and automation agents working through the repository operating model.
- UC-BoK maintainers validating normative models against implementation evidence.

## Product vision

UC Rust is a distributed Unified Commerce platform and reference implementation in which:

- business behavior has one canonical implementation;
- central and edge profiles reuse the same domain and application Operations;
- every external interface is versioned, documented and example-backed;
- infrastructure and specialist capability providers remain replaceable behind governed contracts;
- selected retail processes remain safe during WAN outage;
- synchronization converges without lost or duplicated business effects;
- edge nodes are securely deployed, inventoried, monitored and recovered;
- every quality claim is enforced and evidenced;
- implemented concepts are traceable to a declared UC-BoK revision.

## Value proposition

UC Rust demonstrates that a Unified Commerce platform can combine semantic integrity, operational resilience and provider replaceability without duplicating business logic across channels or deployments. Its value is the combination of:

1. executable validation of UC-BoK;
2. one-place business behavior across adapters and runtime profiles;
3. explicit central/edge authority and convergence guarantees;
4. measurable security, reliability, performance and economics;
5. governed extensibility and capability delegation without vendor leakage into the core.

## Initial delivery strategy

The first executable vertical path is Basket, followed by persistent Basket, commercial calculation and Checkout-to-Order. The foundation must support later touchpoints and capabilities without rewriting business rules.

Initial profiles:

- Central runtime.
- Store-edge runtime.
- Test/reference client or API consumer.

Candidate later profiles:

- Warehouse edge.
- Mobile/associate runtime.
- Self-checkout or smart-checkout adapter.
- SoftPOS-oriented profile.

## In scope for 1.0

- Unified Commerce capability and domain modelling aligned with UC-BoK.
- Basket, pricing/commercial calculation, checkout and order foundations.
- REST and at least one additional delivery adapter over one application core; other adapter forms remain governed extension targets.
- PostgreSQL and SQLite persistence behind stable ports where required by the proving journey.
- Replaceable authorization, payment, fiscal and infrastructure provider boundaries.
- Governed delegation of selected capability Operations to external systems, including pricing/promotion, loyalty, customer enquiry, invoicing and other specialist services where authority, failure, compatibility and evidence are explicit.
- OAuth 2.0/OpenID Connect authentication and Zanzibar-compatible authorization.
- Central and store-edge deployment profiles with explicit offline capability classification.
- Durable synchronization, outbox/inbox, reconciliation and compatibility negotiation.
- Edge fleet desired/actual state, inventory, signed updates, monitoring and rollback.
- Tests, rustdoc, fixtures, security, reliability, performance and economic evidence.
- Cross-repository traceability and feedback with UC-BoK.
- Governed extension points for devices and enterprise integrations.

## Explicit non-goals for 1.0

- Reimplementing a complete ERP, CRM, WMS or enterprise OMS.
- Implementing every UC-BoK capability before the first production-grade journey.
- Building a complete associate or customer superapp UI.
- Owning card data or payment processing that belongs inside certified PSP boundaries.
- Claiming generic offline payment support without method-, provider- and country-specific evidence.
- Mandatory microservice decomposition; modularity and replaceability matter more than service count.
- Vendor-specific business semantics in the domain core.
- Uncontrolled plugins that bypass application, security or quality boundaries.
- Transparent remote delegation that hides authority, latency, partial failure, idempotency or offline consequences.
- AI autonomy without human oversight, measurable evaluation, audit and safe fallback.
- Retail media, influencer attribution, service/repair management, full clienteling and full enterprise fulfillment implementation.
- A custom async executor, consensus system, general-purpose dependency injection container or mandatory broker.

## Build-versus-integrate principles

Build when the capability represents core Unified Commerce semantics, canonical business behavior, distributed consistency or platform governance. Integrate or delegate when a mature specialist system owns the realization or operational authority, including promotion engines, loyalty platforms, customer services, invoicing/fiscal systems, payment processing, external identity providers, specialized AI, devices or enterprise applications.

Every integration or delegated capability must use a stable capability-oriented port and contract tests. Provider types must not leak into the application core. Delegation must preserve the canonical Operation semantics while making provider authority, supported methods, version compatibility, deadlines, idempotency, fallback, offline behavior, observability and economic attribution explicit. Integrating a capability does not remove the obligation to define authority, failure behavior, compatibility and evidence.

## Market evidence dispositions

The detailed assessment is `.context/research/2025-market-guide-impact-assessment.md`. The charter adopts these explicit dispositions:

| Market concern | Charter disposition |
|---|---|
| Modular POS and same source across endpoints | Adopted as an architectural principle. |
| API-first/headless | Adopted; channels reuse canonical Operations rather than duplicate semantics. |
| Central and edge execution | Adopted and required for the 1.0 proving journey. |
| Mobile/associate experience | Candidate runtime/touchpoint; full superapp deferred. |
| OMS and flexible fulfillment | Integrated through contracts; complete enterprise OMS deferred. |
| Clienteling and loyalty | Planned extension capabilities; not required for the 1.0 golden path. |
| Smart checkout, RFID and IoT | Governed device/integration extensions; deferred as core implementations. |
| Offline payment | Provider-, method- and country-specific integration only; no generic guarantee. |
| Analytics and dashboards | Read-model and operational evidence concern; not a source of domain authority. |
| Embedded/agentic AI | Integrated under human oversight, evaluation, audit and fallback controls. |
| Development toolkit/extensibility | Adopted as governed contracts; arbitrary runtime plugins rejected. |
| SoftPOS | Candidate profile, not a 1.0 commitment. |
| Retail media and influencer tracking | Deferred. |
| Disaster recovery, security and privacy | Required product qualities. |

No external market feature becomes scope without an owner, rationale, release, dependencies and measurable acceptance evidence.

## Quality outcomes

Project Ready requires every P0 quality attribute to have:

- a measurable invariant or budget;
- governing ADR/RFC and issue;
- automated enforcement where feasible;
- reproducible evidence;
- accountable owner;
- explicit status and governed exceptions.

No production Rust behavior is accepted without meaningful line and branch coverage, current documentation and evidence proportionate to its risk. Coverage percentage is a guardrail, not a substitute for test quality.

## Product success measures

- 100% of implemented capabilities and public contracts map to stable UC-BoK identifiers or an accepted divergence.
- 100% of production Rust lines and branches are covered by meaningful tests, subject only to an explicit accepted exception.
- Zero duplicated business Operation implementations across delivery profiles.
- Zero prohibited infrastructure dependencies in domain/application crates.
- Zero lost or silently duplicated business effects in supported synchronization failure scenarios.
- Zero silent conflict resolution.
- 100% of managed edge nodes are uniquely identified, inventoried and compatibility-validated.
- 100% of released artifacts are immutable, signed and traceable to a release.
- Critical Operations meet accepted latency, CPU, memory and database budgets.
- Backup, restore, rollback and WAN-partition evidence is reproducible.
- The 1.0 basket-to-order proving journey runs through central and store-edge profiles with declared offline behavior and accepted evidence.

## Assumptions and constraints

- WAN connectivity at retail locations may be absent, intermittent or degraded.
- Central and edge versions will coexist during staged rollouts.
- Retail operations, fiscal obligations and payment risks vary by country and provider.
- UC-BoK evolves independently and can introduce adoption work.
- Rust is the implementation language; UC-BoK remains language-neutral.
- Security and tenant isolation are structural requirements.
- Performance and cost must be measured against representative workloads.
- Provider replaceability does not imply pretending that all providers offer equivalent guarantees.

## Primary risks

- Over-scoping the reference implementation before establishing a vertical golden path.
- Treating distributed edge state as a cache problem rather than an authority and convergence problem.
- Allowing market terminology or vendor features to distort UC-BoK semantics.
- Achieving nominal coverage through weak tests.
- Creating abstractions that hide essential provider capability differences.
- Allowing fleet deployment, migration or authorization versions to drift without compatibility validation.
- Building a generic platform framework not justified by the 1.0 proving journey.
- Treating a remote provider call as if it were a local method and thereby hiding partial failure, latency, authority and offline constraints.

## Terminology

Project terminology is defined in `docs/product/glossary.md`. UC-BoK remains normative; provisional terms and mappings must be reconciled through #38 rather than silently promoted to specification status.

## Change control

Material scope changes require:

- impact assessment against the 1.0 proving journey and release train;
- explicit adopted, integrated, planned, deferred or rejected disposition;
- UC-BoK and Economics by Design impact assessment;
- updates to roadmap, dependencies, budgets and evidence obligations;
- accountable acceptance or a time-bounded waiver.

## Governing documents

- `docs/product/glossary.md`
- `docs/roadmap/uc-rust-1.0-blueprint.md`
- `docs/roadmap/uc-rust-1.0-scope-and-traceability.md`
- `.context/quality-attributes/system-quality-model.md`
- `.context/research/2025-market-guide-impact-assessment.md`
- `governance/uc-bok-traceability.yaml`
- accepted ADRs and RFCs
- GitHub Epic #10 and Project Ready #19