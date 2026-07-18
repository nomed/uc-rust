# Repository Context

## Product

UC Rust is a proprietary, model-driven Unified Commerce platform for retail.

The platform is not intended to reproduce a single vendor product. It implements a coherent commerce kernel that can support multiple channels and touchpoints while preserving shared semantics for customer, product, price, inventory, basket, checkout, order, payment and fulfilment.

## Architectural reference

UC-BoK is the reference model used to connect:

```text
Capability
→ Process and scenario
→ Task and interaction
→ Business object lifecycle
→ Command, event and API
→ Application responsibility
→ Integration pattern
→ KPI and assessment
```

The repository may reference UC-BoK concepts, but runtime code must remain independently buildable and testable.

## Product principles

- Domain ownership must be explicit.
- A module must not access another module's persistence directly.
- Contracts are versioned.
- Money is represented in minor units and never as floating point.
- Historical orders retain their commercial snapshot.
- External effects are idempotent.
- Persistence and event publication will use a transactional outbox.
- Observability, security and operability are part of the baseline, not later additions.
- Store and edge operation must be considered from the beginning, even when first releases are centrally deployed.

## Delivery strategy

Start with a walking skeleton and one end-to-end vertical slice. Avoid premature distribution. The first executable architecture is a Cargo workspace and modular monolith.

## First bounded contexts

- Catalog
- Pricing
- Inventory
- Basket
- Checkout
- Order
- Payment
- Customer

Only Basket and Order are initially represented in code. Other contexts are introduced when required by the vertical slice.

## Definition of done for the foundation

The foundation is considered valid when it provides:

- reproducible Rust toolchain;
- formatting, linting and tests in CI;
- documented architecture decisions;
- pure domain crate without transport or persistence dependencies;
- application crate defining use cases and ports;
- executable health endpoint;
- a tested basket-to-order domain flow.
