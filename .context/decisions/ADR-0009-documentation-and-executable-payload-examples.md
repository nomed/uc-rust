# ADR-0009 — Complete documentation and executable payload examples

- Status: Accepted
- Date: 2026-07-18
- Decision owners: Daniele Favara
- Governing issue: #26

## Context

UC Rust must be understandable without reverse-engineering implementation details. Function and type contracts, DTO shapes and serialized payloads must remain clear to humans and synchronized with executable behavior.

## Decision

All production modules, traits, structs, enums, functions, methods and non-obvious fields require clear and current rustdoc. Documentation must explain, where applicable:

- purpose and responsibility;
- invariants and preconditions;
- input and output semantics;
- errors and failure behavior;
- side effects;
- transaction and idempotency behavior;
- concurrency or ordering assumptions;
- security and tenant implications;
- examples.

Trivial private implementation details may be exempted only by a narrowly defined lint policy. The exemption must not apply to application operations, ports, domain behavior, contracts or public APIs.

Every external input/output DTO and serialized contract requires canonical, realistic and human-readable examples. JSON examples must be pretty-printed. Equivalent human-readable formats may be used when JSON is not the actual contract.

Canonical payload examples should live in versioned fixture files, organized by capability and contract version, and be reused where practical by:

- serialization/deserialization tests;
- application and adapter tests;
- OpenAPI or other contract documentation;
- rustdoc examples;
- integration tests;
- compatibility tests.

Fixtures are executable documentation. CI must parse and validate them. Round-trip tests should deserialize canonical input, exercise the relevant contract or behavior, serialize output and compare semantic content. Snapshot or golden testing may support readability but must not replace explicit assertions for important business behavior.

Examples must be deterministic, free of secrets and personal data, and representative rather than toy placeholders.

## Consequences

- Missing required rustdoc fails CI.
- Contract or behavior changes require documentation and fixture changes in the same pull request.
- Readers can understand a payload directly from the repository.
- Fixtures become part of compatibility governance and must be versioned deliberately.
- Duplicate inline examples should reference or derive from canonical fixtures where tooling permits.

## Recommended structure

```text
fixtures/
  contracts/
    v1/
      basket/
        create-request.valid.json
        create-response.valid.json
        add-line-request.valid.json
        add-line-request.invalid-quantity.json
```

## Alternatives considered

### Inline examples only

Rejected because they are easy to become stale and difficult to reuse across tests and API documentation.

### Generated examples only

Rejected because generated values are often less readable and may not communicate realistic semantics.

### Documentation without executable validation

Rejected because stale documentation would be detected only by manual review.
