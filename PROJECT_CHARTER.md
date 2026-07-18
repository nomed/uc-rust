# UC Rust Project Charter

- Status: Draft for Project Ready review
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
- central and edge profiles reuse the same domain and application operations;
- every external interface is versioned, documented and example-backed;
- infrastructure providers remain replaceable adapters;
- selected retail processes remain safe during WAN outage;
- synchronization converges without lost or duplicated business effects;
- edge nodes are securely deployed, inventoried, monitored and recovered;
- every quality claim is enforced and evidenced;
- implemented concepts are traceable to a declared UC-BoK revision.

## Initial delivery strategy

The first executable vertical path is Basket, followed by persistent Basket, commercial calculation and Checkout-to-Order. The foundation must be strong enough to support later touchpoints and capabilities without rewriting business rules.

Initial profiles:

- Central runtime.
- Store-edge runtime.
- Test/reference client or API consumer.

Candidate later profiles:

- Warehouse edge.
- Mobile/associate runtime.
- Self-checkout or smart-checkout adapter.
- SoftPOS-oriented profile.

## In scope

- Unified Commerce capability and domain modelling aligned with UC-BoK.
- Basket, pricing/commercial calculation, checkout and order foundations.
- REST, gRPC, messaging, worker, job and CLI adapters over one application core.
- PostgreSQL, SQLite and future persistence adapters behind stable ports.
- Replaceable cache, object storage and authorization adapters.
- OAuth 2.0/OpenID Connect authentication and Zanzibar-style authorization.
- Central and edge deployment profiles with explicit offline capability classification.
- Durable synchronization, outbox/inbox, reconciliation and compatibility negotiation.
- Edge fleet desired/actual state, inventory, signed updates, monitoring and rollback.
- Complete tests, rustdoc, fixtures, coverage, security, reliability and performance evidence.
- Cross-repository traceability and feedback with UC-BoK.
- Extension points for devices, payment, OMS, ERP, CRM, loyalty, inventory and AI capabilities.

## Out of scope for the initial implementation

- Reimplementing a complete ERP, CRM, WMS or enterprise OMS.
- Implementing every UC-BoK capability before the first production-grade vertical slice.
- Building a full associate superapp UI before stable domain and API contracts.
- Owning card data or payment processing that should remain within certified PSP boundaries.
- Generic claims of offline payment without provider-specific approval and tests.
- Mandatory microservice decomposition; modularity and replaceability matter more than service count.
- Vendor-specific business semantics in the domain core.
- Uncontrolled plugins that can bypass application, security or quality boundaries.
- AI autonomy without human oversight, measurable evaluation, audit and safe fallback.
- Retail media, influencer attribution and service/repair management in the initial roadmap.

## Build-versus-integrate principles

Build when the capability represents core Unified Commerce semantics, canonical business behavior, distributed consistency or platform governance. Integrate when a mature specialist system owns the capability, including payment processing, external identity providers, specialized AI, devices, tax/fiscal systems or enterprise applications.

Every integration must use a stable capability-oriented port and contract tests. Provider types must not leak into the application core.

## Quality outcomes

Project Ready requires every P0 quality attribute to have:

- measurable invariant or budget;
- governing ADR/RFC and issue;
- automated enforcement;
- reproducible evidence;
- accountable owner;
- explicit status and governed exceptions.

No production Rust behavior is accepted without complete line and branch coverage, meaningful tests and current documentation.

## Product success measures

- 100% of implemented capabilities and public contracts map to stable UC-BoK identifiers or an accepted divergence.
- 100% of production Rust lines and branches are covered by meaningful tests.
- Zero duplicated business operation implementations across delivery profiles.
- Zero prohibited infrastructure dependencies in domain/application crates.
- Zero lost or duplicated business effects in supported synchronization failure scenarios.
- Zero silent conflict resolution.
- 100% of managed edge nodes are uniquely identified, inventoried and compatibility-validated.
- 100% of released artifacts are immutable, signed and traceable to a release.
- Critical operations meet approved latency, CPU, memory and database budgets.
- Backup, restore, rollback and WAN-partition evidence is reproducible.

## Assumptions and constraints

- WAN connectivity at retail locations may be absent, intermittent or degraded.
- Central and edge versions will coexist during staged rollouts.
- Retail operations, fiscal obligations and payment risks vary by country and provider.
- UC-BoK evolves independently and can introduce adoption work.
- Rust is the implementation language; UC-BoK remains language-neutral.
- Security and tenant isolation are structural requirements.
- Performance must be measured against representative workloads.

## Primary risks

- Over-scoping the reference implementation before establishing a vertical golden path.
- Treating distributed edge state as a cache problem rather than an authority and convergence problem.
- Allowing market terminology or vendor features to distort UC-BoK semantics.
- Achieving nominal coverage through weak tests.
- Creating an abstraction that hides essential provider capability differences.
- Allowing fleet deployment, migration or authorization versions to drift independently without compatibility validation.

## Governing documents

- `.context/manifest.yaml`
- `.context/quality-attributes/system-quality-model.md`
- `.context/research/2025-market-guide-impact-assessment.md`
- `governance/uc-bok-traceability.yaml`
- accepted ADRs and RFCs
- GitHub Epic #10 and Project Ready #19
