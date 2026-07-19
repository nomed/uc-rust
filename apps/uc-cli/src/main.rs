//! Command-line composition root for the Runtime Foundation.
//!
//! The binary loads validated configuration, initializes process-local logging,
//! translates CLI arguments into canonical Operation requests and execution context,
//! and renders results as human-readable or JSON output. It owns no business rules:
//! all semantics remain in canonical Operations, including cancellation and deadlines.

use clap::{Parser, Subcommand, ValueEnum};
use std::{path::PathBuf, process::ExitCode, time::Duration};
use tracing_subscriber::EnvFilter;
use uc_config::{load, CliOverrides};
use uc_operation::{
    CancellationToken, ExecutionContext, Operation, OperationError, PingRequest, TraceContext,
};
use uc_runtime::PingOperation;

#[derive(Parser)]
#[command(name = "uc", version, about = "UC Rust Runtime Foundation CLI")]
struct Cli {
    #[arg(long, global = true)]
    config: Option<PathBuf>,
    #[arg(long, global = true)]
    grpc_addr: Option<String>,
    #[arg(long, global = true)]
    gateway_addr: Option<String>,
    #[arg(long, global = true)]
    log_level: Option<String>,
    #[arg(long, global = true, value_enum, default_value_t = Output::Human)]
    output: Output,
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, ValueEnum)]
enum Output {
    Human,
    Json,
}

#[derive(Subcommand)]
enum Command {
    Ping {
        message: String,
        #[arg(long, default_value = "default")]
        tenant: String,
        #[arg(long, default_value = "cli")]
        identity: String,
        #[arg(long, default_value = "cli-request")]
        correlation_id: String,
        #[arg(long)]
        traceparent: Option<String>,
        #[arg(long, default_value_t = 30_000)]
        timeout_ms: u64,
    },
    ServeGrpc,
    ServeGateway,
    Config,
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(exit_code(error.as_ref()))
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let effective = load(
        cli.config.as_deref(),
        CliOverrides {
            grpc_addr: cli.grpc_addr,
            gateway_addr: cli.gateway_addr,
            log_level: cli.log_level,
        },
    )?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(effective.values.log_level.clone()))
        .try_init();

    match cli.command {
        Command::Ping {
            message,
            tenant,
            identity,
            correlation_id,
            traceparent,
            timeout_ms,
        } => {
            let response = PingOperation
                .execute(
                    PingRequest { message },
                    ExecutionContext {
                        tenant_id: tenant,
                        identity,
                        correlation_id,
                        idempotency_key: None,
                        trace: TraceContext {
                            traceparent,
                            tracestate: None,
                        },
                        deadline: Some(
                            std::time::Instant::now() + Duration::from_millis(timeout_ms),
                        ),
                        cancellation: CancellationToken::default(),
                    },
                )
                .await?;
            match cli.output {
                Output::Human => println!(
                    "{} [{} / {}]",
                    response.message, response.tenant_id, response.correlation_id
                ),
                Output::Json => println!("{}", serde_json::to_string(&response)?),
            }
        }
        Command::ServeGrpc => {
            uc_adapters::serve_grpc(effective.values.grpc_addr.parse()?).await?;
        }
        Command::ServeGateway => {
            let endpoint = format!("http://{}", effective.values.grpc_addr);
            uc_adapters::serve_gateway(effective.values.gateway_addr.parse()?, endpoint).await?;
        }
        Command::Config => println!("{}", serde_json::to_string_pretty(&effective)?),
    }

    Ok(())
}

fn exit_code(error: &(dyn std::error::Error + 'static)) -> u8 {
    error
        .downcast_ref::<OperationError>()
        .map_or(1, |error| match error {
            OperationError::InvalidRequest(_) => 2,
            OperationError::Unauthorized | OperationError::Forbidden => 3,
            OperationError::NotFound => 4,
            OperationError::Conflict(_) => 5,
            OperationError::DeadlineExceeded => 6,
            OperationError::Cancelled => 7,
            OperationError::Unavailable => 8,
            OperationError::Internal => 1,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_errors_have_stable_exit_codes() {
        assert_eq!(exit_code(&OperationError::InvalidRequest("x".into())), 2);
        assert_eq!(exit_code(&OperationError::DeadlineExceeded), 6);
        assert_eq!(exit_code(&OperationError::Cancelled), 7);
    }
}
