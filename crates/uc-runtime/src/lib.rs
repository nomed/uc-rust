use async_trait::async_trait;
use uc_operation::{
    CancellationToken, ExecutionContext, Operation, OperationError, OperationId, PingRequest,
    PingResponse, TraceContext,
};

#[derive(Clone, Default)]
pub struct PingOperation;

#[async_trait]
impl Operation for PingOperation {
    const ID: OperationId = OperationId::new("uc.runtime.v1.Ping");
    type Request = PingRequest;
    type Response = PingResponse;

    async fn execute(
        &self,
        request: Self::Request,
        context: ExecutionContext,
    ) -> Result<Self::Response, OperationError> {
        context.ensure_active()?;
        let message = request.message.trim();
        if message.is_empty() {
            return Err(OperationError::InvalidRequest(
                "message must not be blank".into(),
            ));
        }
        context.ensure_active()?;
        Ok(PingResponse {
            message: message.to_owned(),
            tenant_id: context.tenant_id,
            correlation_id: context.correlation_id,
        })
    }
}

pub fn fixture_context(correlation_id: &str) -> ExecutionContext {
    ExecutionContext {
        tenant_id: "tenant-a".into(),
        identity: "fixture".into(),
        correlation_id: correlation_id.into(),
        idempotency_key: None,
        trace: TraceContext {
            traceparent: Some(
                "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01".into(),
            ),
            tracestate: None,
        },
        deadline: None,
        cancellation: CancellationToken::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn ping_preserves_context() {
        let response = PingOperation
            .execute(
                PingRequest {
                    message: "hello".into(),
                },
                fixture_context("corr-1"),
            )
            .await
            .unwrap();
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.correlation_id, "corr-1");
    }

    #[tokio::test]
    async fn expired_deadline_is_rejected_before_execution() {
        let mut context = fixture_context("expired");
        context.deadline = Some(Instant::now() - Duration::from_millis(1));
        let error = PingOperation
            .execute(
                PingRequest {
                    message: "hello".into(),
                },
                context,
            )
            .await
            .unwrap_err();
        assert_eq!(error, OperationError::DeadlineExceeded);
    }

    #[tokio::test]
    async fn caller_cancellation_is_distinct_from_timeout() {
        let context = fixture_context("cancelled");
        context.cancellation.cancel();
        let error = PingOperation
            .execute(
                PingRequest {
                    message: "hello".into(),
                },
                context,
            )
            .await
            .unwrap_err();
        assert_eq!(error, OperationError::Cancelled);
    }
}
