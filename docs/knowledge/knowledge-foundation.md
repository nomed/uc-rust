# UC Rust Knowledge Foundation

- Status: Draft
- Milestone: M0.5 — Knowledge Foundation
- Governing epic: to be assigned
- Date: 2026-07-18

## Purpose

UC Rust treats architecture knowledge as a governed, versioned model rather than as incidental documentation. This foundation defines the language used to describe capabilities, runtime behavior, quality attributes, economics, security, data, interfaces, technologies and releases before implementation begins.

## Layering

```text
Vision
  -> Knowledge model
    -> Architecture records
      -> Capability and runtime design
        -> Implementation
          -> Evidence
```

## Principles

1. Records describe intent, constraints and evidence independently from code structure.
2. Every record has identity, lifecycle, provenance, ownership and typed relations.
3. Accepted, implemented and verified are distinct states.
4. A record may be superseded, but its history is immutable.
5. Cross-record traceability is explicit and machine-readable.
6. UC-BoK provides domain knowledge and normative references; UC Rust records state the project-specific disposition.
7. Yukh may validate and project the model, but the records remain understandable without Yukh.
8. Economics, security, offline behavior and compatibility are first-class concerns.

## Initial record families

- ADR — Architecture Decision Record
- RFC — Request for Comments
- CR — Capability Record
- RRR — Runtime Responsibility Record
- QAR — Quality Attribute Record
- ER — Economic Record
- SR — Security Record
- IR — Interface Record
- DR — Data Record
- TR — Technology Record
- RR — Release Record

The taxonomy is intentionally small. New record families require an ADR demonstrating that an existing type cannot represent the knowledge without semantic loss.

## M0.5 outcome

M0.5 is complete when the record taxonomy, common envelope, lifecycle, relations, identifiers, traceability rules, review process, schemas and first Basket/Runtime examples are accepted and validated.