# Semantic fixture matrix

The canonical Ping fixtures are transport-neutral and compare outcomes rather
than wire encodings.

| Fixture | Canonical result |
|---|---|
| success | trimmed message with tenant and correlation preserved |
| invalid input | `InvalidRequest` |
| expired deadline | `DeadlineExceeded` before Operation work |
| caller cancellation | `Cancelled` before Operation work |

The in-process suite is executable in `crates/uc-runtime/tests`. Adapter tests
verify deterministic gRPC and CLI mappings. Live-process gRPC and REST gateway
execution is tracked as the remaining evidence gap for issue #72 and must be
added before the issue is closed.
