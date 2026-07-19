# Test-Driven Development, Absolute Coverage, and Code Documentation

## Status

Mandatory for the Runtime Foundation and for every subsequent Unified Commerce implementation slice.

## Test-driven development

Production behavior is developed through an explicit red-green-refactor cycle:

1. **Red:** add or change an executable test that describes the required semantic behavior and verify that it fails for the expected reason.
2. **Green:** implement only the smallest production change required to satisfy the test.
3. **Refactor:** improve names, boundaries, duplication, and internal structure while preserving a green test suite.

A pull request must preserve evidence of this reasoning in its description or commit history. Tests added only after an implementation is considered complete do not satisfy this policy. Defects require a failing regression test before the fix.

Tests must assert canonical semantics, not implementation accidents. Transport adapters must reuse shared semantic fixtures wherever equivalent behavior is required.

## Absolute coverage

The governed Runtime Foundation packages must maintain **100% line, function, and region coverage**. Coverage is a merge gate, not an informational metric.

Coverage exclusions are not permitted merely because code is difficult to test. Generated code, compiler-generated paths, and technically unreachable defensive branches may only be excluded through an explicit, reviewed repository policy that records:

- the exact path or expression excluded;
- why executable coverage is impossible or misleading;
- which alternative verification protects the behavior;
- the approving architectural decision.

No such exclusions are approved for the current Runtime Foundation slice.

Coverage does not replace behavioral rigor. Required tests include unit, contract, semantic conformance, adapter integration, cancellation, deadline, error mapping, and architecture tests.

## Documentation of every Rust file

Every governed Rust source file must start with a substantive `//!` module-level documentation block. The block must explain:

- the responsibility of the file/module;
- its architectural layer and allowed dependencies;
- the invariants or semantic guarantees it owns;
- how cancellation, errors, security, and observability apply when relevant;
- important non-goals and extension boundaries.

A filename restated as a one-line comment is not sufficient.

Every public item must have accurate Rustdoc. Public type and function documentation must describe meaning, invariants, failure modes, ownership/lifetime implications where relevant, and examples for non-obvious usage. The workspace `missing_docs = "deny"` lint remains mandatory.

Comments must explain intent and constraints rather than narrating syntax. Stale documentation is a defect and blocks merge.

## Pull-request evidence

A Runtime Foundation pull request is ready for review only when it contains:

- the behavior-first test plan and red-green-refactor evidence;
- 100% line/function/region coverage evidence from CI;
- shared semantic fixtures for equivalent adapter behavior;
- successful Rustdoc generation with warnings denied;
- successful repository documentation-structure checks;
- a statement of any remaining gap. A known required gap keeps the pull request in draft.

## Enforcement

The `Runtime Foundation` workflow enforces:

- compilation and Clippy with warnings denied;
- module-level documentation for every Rust file under `crates/` and `apps/`;
- Rustdoc generation with warnings denied;
- tests under `cargo-llvm-cov`;
- 100% line, function, and region coverage;
- architecture and protobuf compatibility checks.

These gates may be strengthened but may not be weakened inside an implementation pull request without an accepted governance decision.
