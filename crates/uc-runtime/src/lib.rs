use async_trait::async_trait;
use uc_operation::{ExecutionContext, Operation, OperationError, OperationId, PingRequest, PingResponse};

#[derive(Clone, Default)]
pub struct PingOperation;

#[async_trait]
impl Operation for PingOperation {
    const ID: OperationId = OperationId::new("uc.runtime.v1.Ping");
    type Request = PingRequest;
    type Response = PingResponse;

    async fn execute(&self, request: Self::Request, context: ExecutionContext) -> Result<Self::Response, OperationError> {
        context.ensure_active()?;
        let message = request.message.trim();
        if message.is_empty() {
            return Err(OperationError::InvalidRequest("message must not be blank".into()));
        }
        Ok(PingResponse { message: message.to_owned(), tenant_id: context.tenant_id, correlation_id: context.correlation_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ping_preserves_context() {
        let response = PingOperation.execute(
            PingRequest { message: "hello".into() },
            ExecutionContext { tenant_id: "tenant-a".into(), identity: "test".into(), correlation_id: "corr-1".into(), idempotency_key: None, deadline: None },
        ).await.unwrap();
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.correlation_id, "corr-1");
    }
}
