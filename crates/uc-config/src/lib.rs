use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::BTreeMap, fs, path::Path};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub grpc_addr: String,
    pub gateway_addr: String,
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { grpc_addr: "127.0.0.1:50051".into(), gateway_addr: "127.0.0.1:8080".into(), log_level: "info".into() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CliOverrides {
    pub grpc_addr: Option<String>,
    pub gateway_addr: Option<String>,
    pub log_level: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct EffectiveSettings {
    pub values: Settings,
    pub provenance: BTreeMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid TOML configuration: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("invalid effective configuration: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid socket address in {field}: {value}")]
    InvalidAddress { field: &'static str, value: String },
    #[error("unsupported log level: {0}")]
    InvalidLogLevel(String),
}

pub fn load(path: Option<&Path>, cli: CliOverrides) -> Result<EffectiveSettings, ConfigError> {
    let mut value = serde_json::to_value(Settings::default())?;
    let mut provenance = defaults_provenance();

    if let Some(path) = path {
        let text = fs::read_to_string(path)?;
        let file_layer: Map<String, Value> = toml::from_str::<toml::Value>(&text)?
            .try_into()
            .map_err(ConfigError::Json)?;
        merge_layer(&mut value, file_layer, &format!("file:{}", path.display()), &mut provenance);
    }

    merge_layer(&mut value, environment_layer(), "environment", &mut provenance);
    merge_layer(&mut value, cli_layer(cli), "cli", &mut provenance);

    let settings: Settings = serde_json::from_value(value)?;
    validate(&settings)?;
    Ok(EffectiveSettings { values: settings, provenance })
}

fn environment_layer() -> Map<String, Value> {
    let mut layer = Map::new();
    for (key, field) in [("UC_GRPC_ADDR", "grpc_addr"), ("UC_GATEWAY_ADDR", "gateway_addr"), ("UC_LOG_LEVEL", "log_level")] {
        if let Ok(value) = std::env::var(key) { layer.insert(field.into(), Value::String(value)); }
    }
    layer
}

fn cli_layer(cli: CliOverrides) -> Map<String, Value> {
    let mut layer = Map::new();
    if let Some(v) = cli.grpc_addr { layer.insert("grpc_addr".into(), Value::String(v)); }
    if let Some(v) = cli.gateway_addr { layer.insert("gateway_addr".into(), Value::String(v)); }
    if let Some(v) = cli.log_level { layer.insert("log_level".into(), Value::String(v)); }
    layer
}

fn merge_layer(target: &mut Value, layer: Map<String, Value>, source: &str, provenance: &mut BTreeMap<String, String>) {
    let target = target.as_object_mut().expect("settings serialize as object");
    for (key, value) in layer { target.insert(key.clone(), value); provenance.insert(key, source.into()); }
}

fn defaults_provenance() -> BTreeMap<String, String> {
    ["grpc_addr", "gateway_addr", "log_level"].into_iter().map(|key| (key.into(), "default".into())).collect()
}

fn validate(settings: &Settings) -> Result<(), ConfigError> {
    for (field, value) in [("grpc_addr", &settings.grpc_addr), ("gateway_addr", &settings.gateway_addr)] {
        value.parse::<std::net::SocketAddr>().map_err(|_| ConfigError::InvalidAddress { field, value: value.clone() })?;
    }
    if !matches!(settings.log_level.as_str(), "trace" | "debug" | "info" | "warn" | "error") {
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
        fs::write(&path, "grpc_addr='127.0.0.1:51000'\ngateway_addr='127.0.0.1:8100'\nlog_level='debug'\n").unwrap();
        unsafe { std::env::set_var("UC_GRPC_ADDR", "127.0.0.1:52000") };
        let effective = load(Some(&path), CliOverrides { grpc_addr: Some("127.0.0.1:53000".into()), ..Default::default() }).unwrap();
        unsafe { std::env::remove_var("UC_GRPC_ADDR") };
        assert_eq!(effective.values.grpc_addr, "127.0.0.1:53000");
        assert_eq!(effective.values.gateway_addr, "127.0.0.1:8100");
        assert_eq!(effective.provenance["grpc_addr"], "cli");
    }
}
