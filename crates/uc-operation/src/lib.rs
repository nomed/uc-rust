use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct OperationId(&'static str);

impl OperationId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TraceContext {
    pub traceparent: Option<String>,
    pub tracestate: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub tenant_id: String,
    pub identity: String,
    pub correlation_id: String,
    pub idempotency_key: Option<String>,
    pub trace: TraceContext,
    pub deadline: Option<Instant>,
    pub cancellation: CancellationToken,
}

impl ExecutionContext {
    pub fn ensure_active(&self) -> Result<(), OperationError> {
        if self.cancellation.is_cancelled() {
            return Err(OperationError::Cancelled);
        }
        if self.deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Err(OperationError::DeadlineExceeded);
        }
        Ok(())
    }

    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }
}

#[derive(Debug, Error, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum OperationError {
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("deadline exceeded")]
    DeadlineExceeded,
    #[error("operation cancelled")]
    Cancelled,
    #[error("service unavailable")]
    Unavailable,
    #[error("internal operation failure")]
    Internal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PingRequest {
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PingResponse {
    pub message: String,
    pub tenant_id: String,
    pub correlation_id: String,
}

#[async_trait]
pub trait Operation: Send + Sync + 'static {
    const ID: OperationId;
    type Request: Send + Sync;
    type Response: Send + Sync;

    async fn execute(
        &self,
        request: Self::Request,
        context: ExecutionContext,
    ) -> Result<Self::Response, OperationError>;
}
