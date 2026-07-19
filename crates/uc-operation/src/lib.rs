//! Canonical, transport-neutral Operation contracts for the Unified Commerce runtime.
//!
//! This crate belongs to the application boundary. It defines invocation identity,
//! execution context, cancellation, deadlines, trace propagation, canonical errors,
//! and request/response DTOs without depending on HTTP, gRPC, CLI, or telemetry SDKs.
//! Adapters translate delivery-specific evidence into these types. Operations must
//! cooperate with cancellation and deadlines and must never expose provider errors.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use thiserror::Error;

/// Stable identifier of a canonical Operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct OperationId(&'static str);

impl OperationId {
    /// Creates an identifier from a process-lifetime static string.
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Returns the canonical textual identifier.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Transport-neutral W3C trace propagation fields accepted at delivery boundaries.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TraceContext {
    /// W3C `traceparent` value, preserved without interpreting business payloads.
    pub traceparent: Option<String>,
    /// Optional W3C `tracestate` value.
    pub tracestate: Option<String>,
}

/// Cloneable cooperative cancellation signal shared with an Operation invocation.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Requests cancellation. The call is idempotent and visible to all clones.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Reports whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// Immutable invocation metadata supplied explicitly to a canonical Operation.
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    /// Tenant whose capability is being invoked.
    pub tenant_id: String,
    /// Authenticated or technical caller identity.
    pub identity: String,
    /// Safe diagnostic correlation identifier.
    pub correlation_id: String,
    /// Optional idempotency key supplied by the caller.
    pub idempotency_key: Option<String>,
    /// Trace propagation evidence received at the delivery boundary.
    pub trace: TraceContext,
    /// Absolute monotonic deadline for admitting and continuing work.
    pub deadline: Option<Instant>,
    /// Cooperative cancellation signal for the invocation.
    pub cancellation: CancellationToken,
}

impl ExecutionContext {
    /// Rejects work that is cancelled or whose deadline has elapsed.
    pub fn ensure_active(&self) -> Result<(), OperationError> {
        if self.cancellation.is_cancelled() {
            return Err(OperationError::Cancelled);
        }
        if self
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            return Err(OperationError::DeadlineExceeded);
        }
        Ok(())
    }

    /// Returns the saturating duration remaining before the deadline.
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }
}

/// Canonical failure categories exposed by Operations to every adapter.
#[derive(Debug, Error, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum OperationError {
    /// Request violates the canonical input contract.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// Caller has not established an authenticated identity.
    #[error("unauthorized")]
    Unauthorized,
    /// Authenticated caller lacks permission.
    #[error("forbidden")]
    Forbidden,
    /// Requested canonical resource does not exist.
    #[error("not found")]
    NotFound,
    /// Request conflicts with current canonical state.
    #[error("conflict: {0}")]
    Conflict(String),
    /// Invocation deadline elapsed before completion.
    #[error("deadline exceeded")]
    DeadlineExceeded,
    /// Caller or runtime explicitly cancelled the invocation.
    #[error("operation cancelled")]
    Cancelled,
    /// Required realization is temporarily unavailable.
    #[error("service unavailable")]
    Unavailable,
    /// Internal failure whose implementation details must remain private.
    #[error("internal operation failure")]
    Internal,
}

/// Canonical Ping request used by semantic conformance fixtures.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PingRequest {
    /// Message to validate, normalize, and echo.
    pub message: String,
}

/// Canonical Ping response independent of transport serialization.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PingResponse {
    /// Trimmed canonical message.
    pub message: String,
    /// Tenant copied from the execution context.
    pub tenant_id: String,
    /// Correlation identifier copied from the execution context.
    pub correlation_id: String,
}

/// Asynchronous canonical application Operation.
#[async_trait]
pub trait Operation: Send + Sync + 'static {
    /// Stable canonical identifier used for diagnostics and realization attribution.
    const ID: OperationId;
    /// Transport-neutral request type.
    type Request: Send + Sync;
    /// Transport-neutral response type.
    type Response: Send + Sync;

    /// Executes canonical semantics under the supplied invocation context.
    async fn execute(
        &self,
        request: Self::Request,
        context: ExecutionContext,
    ) -> Result<Self::Response, OperationError>;
}
