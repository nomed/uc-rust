use std::time::{Duration, Instant};
use uc_operation::{Operation, OperationError, PingRequest};
use uc_runtime::{fixture_context, PingOperation};

#[tokio::test]
async fn success_fixture_is_canonical() {
    let response = PingOperation
        .execute(
            PingRequest {
                message: " hello ".into(),
            },
            fixture_context("fixture-success"),
        )
        .await
        .unwrap();
    assert_eq!(response.message, "hello");
    assert_eq!(response.tenant_id, "tenant-a");
    assert_eq!(response.correlation_id, "fixture-success");
}

#[tokio::test]
async fn invalid_input_fixture_is_canonical() {
    let error = PingOperation
        .execute(
            PingRequest {
                message: "   ".into(),
            },
            fixture_context("fixture-invalid"),
        )
        .await
        .unwrap_err();
    assert_eq!(
        error,
        OperationError::InvalidRequest("message must not be blank".into())
    );
}

#[tokio::test]
async fn expired_deadline_fixture_is_canonical() {
    let mut context = fixture_context("fixture-timeout");
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
async fn caller_cancellation_fixture_is_canonical() {
    let context = fixture_context("fixture-cancelled");
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
