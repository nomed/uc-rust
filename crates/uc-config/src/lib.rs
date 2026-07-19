//! Typed Runtime Foundation configuration and provenance tracking.
//!
//! This crate is the only governed boundary allowed to read process environment
//! variables. It composes immutable effective settings using the precedence
//! `defaults < file < environment < CLI`, validates them before returning, and
//! records the winning source for each field. It must not initialize transports,
//! telemetry SDKs, or application Operations.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};
use thiserror::Error;

/// Validated runtime settings consumed by the composition root.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    /// Socket address on which the gRPC adapter listens.
    pub grpc_addr: String,
    /// Socket address on which the REST gateway listens.
    pub gateway_addr: String,
    /// Structured logging filter level.
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            grpc_addr: "127.0.0.1:50051".into(),
            gateway_addr: "127.0.0.1:8080".into(),
            log_level: "info".into(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct PartialSettings {
    grpc_addr: Option<String>,
    gateway_addr: Option<String>,
    log_level: Option<String>,
}

/// Optional command-line overrides applied after file and environment layers.
#[derive(Clone, Debug, Default)]
pub struct CliOverrides {
    /// Optional gRPC listen address override.
    pub grpc_addr: Option<String>,
    /// Optional REST gateway listen address override.
    pub gateway_addr: Option<String>,
    /// Optional logging level override.
    pub log_level: Option<String>,
}

/// Validated settings together with field-level winning-source provenance.
#[derive(Clone, Debug, Serialize)]
pub struct EffectiveSettings {
    /// Final validated values.
    pub values: Settings,
    /// Winning source for every configurable field.
    pub provenance: BTreeMap<String, String>,
}

/// Failures produced while loading or validating runtime configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Configuration file could not be read.
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    /// Configuration file is not valid TOML for the known schema.
    #[error("invalid TOML configuration: {0}")]
    Toml(#[from] toml::de::Error),
    /// A configured listener address is not a valid socket address.
    #[error("invalid socket address in {field}: {value}")]
    InvalidAddress {
        /// Name of the invalid field.
        field: &'static str,
        /// Rejected value.
        value: String,
    },
    /// Logging level is outside the supported allow-list.
    #[error("unsupported log level: {0}")]
    InvalidLogLevel(String),
}

/// Loads, composes, validates, and returns effective runtime configuration.
///
/// Environment access is intentionally confined to this function's private helpers.
pub fn load(path: Option<&Path>, cli: CliOverrides) -> Result<EffectiveSettings, ConfigError> {
    let mut settings = Settings::default();
    let mut provenance = defaults_provenance();

    if let Some(path) = path {
        let layer: PartialSettings = toml::from_str(&fs::read_to_string(path)?)?;
        apply_layer(
            &mut settings,
            layer,
            &format!("file:{}", path.display()),
            &mut provenance,
        );
    }

    apply_layer(
        &mut settings,
        environment_layer(),
        "environment",
        &mut provenance,
    );
    apply_layer(
        &mut settings,
        PartialSettings {
            grpc_addr: cli.grpc_addr,
            gateway_addr: cli.gateway_addr,
            log_level: cli.log_level,
        },
        "cli",
        &mut provenance,
    );

    validate(&settings)?;
    Ok(EffectiveSettings {
        values: settings,
        provenance,
    })
}

fn environment_layer() -> PartialSettings {
    PartialSettings {
        grpc_addr: std::env::var("UC_GRPC_ADDR").ok(),
        gateway_addr: std::env::var("UC_GATEWAY_ADDR").ok(),
        log_level: std::env::var("UC_LOG_LEVEL").ok(),
    }
}

fn apply_layer(
    target: &mut Settings,
    layer: PartialSettings,
    source: &str,
    provenance: &mut BTreeMap<String, String>,
) {
    if let Some(value) = layer.grpc_addr {
        target.grpc_addr = value;
        provenance.insert("grpc_addr".into(), source.into());
    }
    if let Some(value) = layer.gateway_addr {
        target.gateway_addr = value;
        provenance.insert("gateway_addr".into(), source.into());
    }
    if let Some(value) = layer.log_level {
        target.log_level = value;
        provenance.insert("log_level".into(), source.into());
    }
}

fn defaults_provenance() -> BTreeMap<String, String> {
    ["grpc_addr", "gateway_addr", "log_level"]
        .into_iter()
        .map(|key| (key.into(), "default".into()))
        .collect()
}

fn validate(settings: &Settings) -> Result<(), ConfigError> {
    for (field, value) in [
        ("grpc_addr", &settings.grpc_addr),
        ("gateway_addr", &settings.gateway_addr),
    ] {
        value
            .parse::<std::net::SocketAddr>()
            .map_err(|_| ConfigError::InvalidAddress {
                field,
                value: value.clone(),
            })?;
    }
    if !matches!(
        settings.log_level.as_str(),
        "trace" | "debug" | "info" | "warn" | "error"
    ) {
        return Err(ConfigError::InvalidLogLevel(settings.log_level.clone()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn precedence_is_default_file_environment_cli() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        fs::write(
            &path,
            "grpc_addr='127.0.0.1:51000'\ngateway_addr='127.0.0.1:8100'\nlog_level='debug'\n",
        )
        .unwrap();
        unsafe { std::env::set_var("UC_GRPC_ADDR", "127.0.0.1:52000") };
        let effective = load(
            Some(&path),
            CliOverrides {
                grpc_addr: Some("127.0.0.1:53000".into()),
                ..Default::default()
            },
        )
        .unwrap();
        unsafe { std::env::remove_var("UC_GRPC_ADDR") };
        assert_eq!(effective.values.grpc_addr, "127.0.0.1:53000");
        assert_eq!(effective.values.gateway_addr, "127.0.0.1:8100");
        assert_eq!(effective.provenance["grpc_addr"], "cli");
    }

    #[test]
    #[serial]
    fn partial_file_inherits_defaults() {
        unsafe {
            std::env::remove_var("UC_GRPC_ADDR");
            std::env::remove_var("UC_GATEWAY_ADDR");
            std::env::remove_var("UC_LOG_LEVEL");
        }
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        fs::write(&path, "log_level='warn'\n").unwrap();
        let effective = load(Some(&path), CliOverrides::default()).unwrap();
        assert_eq!(effective.values.grpc_addr, "127.0.0.1:50051");
        assert_eq!(effective.values.log_level, "warn");
    }
}
