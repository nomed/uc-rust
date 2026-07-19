# Runtime Foundation Coverage Inventory

## Status

Baseline captured from Runtime Foundation workflow run `29679964407` for pull request #79.
The baseline predates the live gRPC RED test and is intentionally below the mandatory 100% merge gate.

## Aggregate baseline

| Metric | Covered | Total | Coverage |
|---|---:|---:|---:|
| Lines | 283 | 531 | 53.30% |
| Functions | 26 | 60 | 43.33% |
| Regions | below gate | — | below 100% |

The workflow reported eleven passing tests. Compilation, Clippy, module documentation,
Rustdoc and protobuf checks passed; absolute coverage failed as designed.

## Coverage debt by governed module

### `apps/uc-cli/src/main.rs`

Uncovered production behavior includes:

- CLI parsing and generated command-dispatch paths;
- process entry and `run` orchestration;
- configuration loading failures;
- Ping execution through human and JSON output modes;
- gRPC server and REST gateway command dispatch;
- complete canonical error-to-exit-code mapping.

Representative uncovered source ranges: `61-69`, `71-80`, `82-93`, `95-119`,
`123`, `126-127`, `129`, `132-133`, `140-142`, `145-146`.

### `crates/uc-adapters/src/lib.rs`

Uncovered production behavior includes:

- real gRPC request decode, execution and response encode;
- metadata extraction and propagation;
- server startup paths;
- REST gateway startup and live REST-to-gRPC invocation;
- safe problem-details construction;
- complete gRPC-to-HTTP status mapping;
- complete canonical-error-to-gRPC status mapping.

Representative uncovered source ranges: `47-83`, `86-91`, `97-102`, `137-143`,
`145-182`, `184-209`, `211-224`, `226-239`.

### `crates/uc-config/src/lib.rs`

Uncovered behavior includes environment-layer variants and validation failures,
particularly invalid socket addresses and unsupported log levels.

Representative uncovered source lines: `100`, `169-170`, `177`.

### `crates/uc-operation/src/lib.rs`

Uncovered behavior includes `OperationId::new`, `OperationId::as_str`,
`ExecutionContext::remaining` with and without deadlines, and remaining branches of
admission checks.

Representative uncovered source ranges: `26-33`, `98-101`.

## Ordered remediation plan

1. Add live gRPC semantic-conformance tests, beginning with invalid input and safe
   correlation metadata.
2. Reuse the same canonical fixtures for success, expired deadline, caller
   cancellation and trace propagation.
3. Add live REST gateway tests that reach the real gRPC server.
4. Complete deterministic gRPC, REST and CLI error-mapping tables.
5. Cover configuration validation and provenance branches.
6. Cover transport-neutral operation identifiers, remaining deadline computation and
   cancellation admission branches.
7. Add deterministic span-sink assertions for `decode → invocation → operation → encode`.
8. Repeat the inventory after every green/refactor cycle until line, function and region
   coverage all report 100%.

## Evidence policy

Every remediation item starts with a failing executable test and follows
red → green → refactor. Coverage-only tests that assert implementation accidents are
not acceptable. Generated-code exclusions require a separately accepted governance
decision; none is approved for this slice.
