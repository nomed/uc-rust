use clap::{Parser, Subcommand, ValueEnum};
use std::{path::PathBuf, time::Duration};
use tracing_subscriber::EnvFilter;
use uc_config::{load, CliOverrides};
use uc_operation::{ExecutionContext, Operation, PingRequest};
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
    },
    ServeGrpc,
    ServeGateway,
    Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        } => {
            let response = PingOperation
                .execute(
                    PingRequest { message },
                    ExecutionContext {
                        tenant_id: tenant,
                        identity,
                        correlation_id,
                        idempotency_key: None,
                        deadline: Some(std::time::Instant::now() + Duration::from_secs(30)),
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
