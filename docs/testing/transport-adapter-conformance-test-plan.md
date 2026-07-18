# Transport Adapter Conformance Test Plan

- Governing issue: #51
- Governing decision: ADR-0028

## Required proof

Use one canonical Operation and the same canonical fixtures through at least two adapter forms among REST, gRPC, CLI and worker.

Verify:

- identical Operation ID and semantic version;
- identical canonical input after decoding;
- identical canonical disposition and error class;
- identity, tenant, correlation, deadline, cancellation and idempotency propagation;
- adapter-only validation for malformed protocol data;
- application validation for canonical and business rules;
- safe status and error mapping;
- no provider or persistence diagnostics in public responses;
- no adapter selection of Capability Realizations;
- no repository, transaction manager, provider SDK or business-event publisher imports in adapters;
- adapter revision compatibility independent from Operation semantic version;
- trace and economic evidence separating adapter overhead from Operation and realization cost.

## Adapter coverage

REST tests method, path, media type, headers, body mapping and HTTP status views.

gRPC tests protobuf compatibility, deadline/cancellation propagation and status details.

CLI tests arguments, input streams, stable exit codes and machine-readable output.

Worker tests Work Envelope mapping, durability-point acknowledgement, retries, quarantine and reconciliation under ADR-0027.

## Evidence bundle

The M1 bundle contains shared canonical fixtures, transport representation fixtures, mapping tables, test output, dependency checks, compatibility matrix and performance/economic scorecard.