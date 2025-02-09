// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::RoochOpt;

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Parser)]
#[serde(deny_unknown_fields)]
pub struct DAConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(name = "internal-da-server", long, help = "internal da server config")]
    pub internal_da_server: Option<InternalDAServerConfig>,
    // TODO external da server config
    // TODO internal external policy
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DAServerSubmitStrategy {
    All,
    // >= n/2+1
    Quorum,
    // >= number
    Number(usize),
}

impl FromStr for DAServerSubmitStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(DAServerSubmitStrategy::All),
            "quorum" => Ok(DAServerSubmitStrategy::Quorum),
            _ => {
                if let Ok(n) = s.parse::<usize>() {
                    Ok(DAServerSubmitStrategy::Number(n))
                } else {
                    Err(format!("invalid da server submit strategy: {}", s))
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InternalDAServerConfigType {
    Celestia(DAServerCelestiaConfig),
    OpenDA(DAServerOpenDAConfig),
}

impl FromStr for InternalDAServerConfigType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Value =
            serde_json::from_str(s).map_err(|e| format!("Error parsing JSON: {}, {}", e, s))?;

        if let Some(obj) = v.as_object() {
            if let Some(celestia) = obj.get("celestia") {
                let celestia_config: DAServerCelestiaConfig =
                    serde_json::from_value(celestia.clone()).map_err(|e| {
                        format!(
                            "invalid celestia config: {} error: {}, original: {}",
                            celestia, e, s
                        )
                    })?;
                Ok(InternalDAServerConfigType::Celestia(celestia_config))
            } else if let Some(openda) = obj.get("open-da") {
                let openda_config: DAServerOpenDAConfig = serde_json::from_value(openda.clone())
                    .map_err(|e| {
                        format!(
                            "invalid open-da config: {}, error: {}, original: {}",
                            openda, e, s
                        )
                    })?;
                Ok(InternalDAServerConfigType::OpenDA(openda_config))
            } else {
                Err(format!("Invalid value: {}", s))
            }
        } else {
            Err(format!("Invalid value: {}", s))
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Parser)]
#[serde(deny_unknown_fields)]
pub struct InternalDAServerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(
        name = "submit-strategy",
        long,
        help = "specifies the submission strategy of internal DA servers to be used. 'all' with all servers, 'quorum' with quorum servers, 'n' with n servers, etc."
    )]
    pub submit_strategy: Option<DAServerSubmitStrategy>,
    #[clap(
        name = "servers",
        long,
        help = "specifies the type of internal DA servers to be used. 'celestia' with corresponding Celestia server configuration, 'xxx' with corresponding xxx server configuration, etc."
    )]
    pub servers: Vec<InternalDAServerConfigType>,
}

impl InternalDAServerConfig {
    pub fn adjust_submit_strategy(&mut self) {
        let servers_count = self.servers.len();

        // Set default strategy to All if it's None.
        let strategy = self
            .submit_strategy
            .get_or_insert(DAServerSubmitStrategy::All);

        // If it's a Number, adjust the value to be within [1, n].
        if let DAServerSubmitStrategy::Number(ref mut num) = strategy {
            *num = std::cmp::max(1, std::cmp::min(*num, servers_count));
        }
    }

    pub fn calculate_submit_threshold(&mut self) -> usize {
        self.adjust_submit_strategy(); // Make sure submit_strategy is adjusted before calling this function.

        let servers_count = self.servers.len();
        match self.submit_strategy {
            Some(DAServerSubmitStrategy::All) => servers_count,
            Some(DAServerSubmitStrategy::Quorum) => servers_count / 2 + 1,
            Some(DAServerSubmitStrategy::Number(number)) => number,
            None => servers_count, // Default to 'All' if submit_strategy is None
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenDAScheme {
    // gcs(Google Could Service) config:
    // bucket
    // root
    // credential
    // predefined_acl
    // default_storage_class
    #[default]
    GCS,
    // s3 config:
    // root
    // bucket
    // region
    // endpoint
    // access_key_id
    // secret_access_key
    S3,
}

impl Display for OpenDAScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenDAScheme::GCS => write!(f, "gcs"),
            OpenDAScheme::S3 => write!(f, "s3"),
        }
    }
}

impl FromStr for OpenDAScheme {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gcs" => Ok(OpenDAScheme::GCS),
            "s3" => Ok(OpenDAScheme::S3),
            _ => Err("open-da scheme no match"),
        }
    }
}

fn parse_hashmap(
    s: &str,
) -> Result<HashMap<String, String>, Box<dyn Error + Send + Sync + 'static>> {
    s.split(',')
        .filter(|kv| !kv.is_empty())
        .map(|kv| {
            let mut parts = kv.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) if !key.trim().is_empty() => {
                    Ok((key.to_string(), value.to_string()))
                }
                (Some(""), Some(_)) => Err("key is missing before '='".into()),
                _ => {
                    Err("each key=value pair must be separated by a comma and contain a key".into())
                }
            }
        })
        .collect()
}

// test parse_hashmap

// Open DA provides ability to access various storage services
#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Parser)]
#[serde(deny_unknown_fields)]
pub struct DAServerOpenDAConfig {
    #[clap(
        name = "scheme",
        long,
        value_enum,
        default_value = "gcs",
        help = "specifies the type of storage service to be used. 'gcs' with corresponding GCS server configuration, 's3' with corresponding S3 server configuration, etc."
    )]
    pub scheme: OpenDAScheme,
    #[clap(
    name = "config",
    long,
    value_parser = parse_hashmap,
    help = "specifies the configuration of the storage service. 'gcs' with corresponding GCS server configuration, 's3' with corresponding S3 server configuration, etc."
    )]
    pub config: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(
        name = "max-segment-size",
        long,
        help = "max segment size, striking a balance between throughput and the constraints on blob size."
    )]
    pub max_segment_size: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Parser)]
#[serde(deny_unknown_fields)]
pub struct DAServerCelestiaConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(
        name = "namespace",
        long,
        env = "DA_CELESTIA_NAMESPACE",
        help = "celestia namespace"
    )]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(
        name = "conn",
        long,
        env = "DA_CELESTIA_CONN",
        help = "celestia node connection"
    )]
    pub conn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(
        name = "auth-token",
        long,
        env = "DA_CELESTIA_AUTH_TOKEN",
        help = "celestia node auth token"
    )]
    pub auth_token: Option<String>,
    // for celestia:
    // support for up to 8 MB blocks, starting with 2MB at genesis and upgradeable through onchain governance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(
        name = "max-segment-size",
        long,
        env = "DA_CELESTIA_MAX_SEGMENT_SIZE",
        help = "max segment size, striking a balance between throughput and the constraints on blob size."
    )]
    pub max_segment_size: Option<u64>,
}

impl Default for DAServerCelestiaConfig {
    fn default() -> Self {
        Self {
            namespace: None,
            conn: None,
            auth_token: None,
            max_segment_size: Some(1024 * 1024),
        }
    }
}

impl DAServerCelestiaConfig {
    pub fn new_with_defaults(mut self) -> Self {
        let default = DAServerCelestiaConfig::default();
        if self.max_segment_size.is_none() {
            self.max_segment_size = default.max_segment_size;
        }
        self
    }
}

impl Config for DAConfig {}

impl FromStr for DAConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deserialized = serde_json::from_str(s)?;
        Ok(deserialized)
    }
}

impl FromStr for InternalDAServerConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deserialized = serde_json::from_str(s)?;
        Ok(deserialized)
    }
}

impl DAConfig {
    pub fn merge_with_opt(&mut self, opt: &RoochOpt) -> anyhow::Result<()> {
        if let Some(ref da_config) = opt.da {
            // TODO merge with field checking
            *self = da_config.clone();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_submit_strategy_default_to_all() {
        let mut config = InternalDAServerConfig {
            submit_strategy: None,
            servers: vec![], // Empty for this test
        };
        config.adjust_submit_strategy();
        assert_eq!(config.submit_strategy, Some(DAServerSubmitStrategy::All));
    }

    #[test]
    fn test_adjust_submit_strategy_number_too_low() {
        let mut config = InternalDAServerConfig {
            submit_strategy: Some(DAServerSubmitStrategy::Number(0)),
            servers: vec![
                InternalDAServerConfigType::Celestia(DAServerCelestiaConfig::default());
                2
            ], // Two servers for this test
        };
        config.adjust_submit_strategy();
        assert_eq!(
            config.submit_strategy,
            Some(DAServerSubmitStrategy::Number(1))
        );
    }

    #[test]
    fn test_adjust_submit_strategy_number_too_high() {
        let mut config = InternalDAServerConfig {
            submit_strategy: Some(DAServerSubmitStrategy::Number(5)),
            servers: vec![
                InternalDAServerConfigType::Celestia(DAServerCelestiaConfig::default());
                3
            ], // Three servers for this test
        };
        config.adjust_submit_strategy();
        assert_eq!(
            config.submit_strategy,
            Some(DAServerSubmitStrategy::Number(3))
        );
    }

    #[test]
    fn test_adjust_submit_strategy_number_within_range() {
        let mut config = InternalDAServerConfig {
            submit_strategy: Some(DAServerSubmitStrategy::Number(2)),
            servers: vec![
                InternalDAServerConfigType::Celestia(DAServerCelestiaConfig::default());
                4
            ], // Four servers for this test
        };
        config.adjust_submit_strategy();
        assert_eq!(
            config.submit_strategy,
            Some(DAServerSubmitStrategy::Number(2))
        );
    }

    #[test]
    fn test_parse_hashmap_ok() {
        let input = "key1=VALUE1,key2=value2";
        let output = parse_hashmap(input).unwrap();

        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "VALUE1".to_string());
        expected.insert("key2".to_string(), "value2".to_string());

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_hashmap_empty_value() {
        let input = "key1=,key2=value2";
        let output = parse_hashmap(input).unwrap();

        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "".to_string());
        expected.insert("key2".to_string(), "value2".to_string());

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_hashmap_empty_string() {
        let input = "";
        let output = parse_hashmap(input).unwrap();

        let expected = HashMap::new();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_hashmap_missing_value() {
        let input = "key1,key2=value2";
        let output = parse_hashmap(input);

        assert!(output.is_err());
    }

    #[test]
    fn test_parse_hashmap_missing_key() {
        let input = "=value1,key2=value2";
        let output = parse_hashmap(input);

        assert!(output.is_err());
    }

    #[test]
    fn test_parse_hashmap_no_equals_sign() {
        let input = "key1value1,key2=value2";
        let output = parse_hashmap(input);

        assert!(output.is_err());
    }

    #[test]
    fn test_internal_da_server_config_str() {
        let celestia_config_str = r#"{"celestia": {"namespace": "test_namespace", "conn": "test_conn", "auth_token": "test_token", "max_segment_size": 2048}}"#;
        let openda_config_str = r#"{"open-da": {"scheme": "gcs", "config": {"Param1": "value1", "param2": "Value2"}, "max_segment_size": 2048}}"#;
        let invalid_config_str = r#"{"unknown": {...}}"#;

        match InternalDAServerConfigType::from_str(celestia_config_str) {
            Ok(InternalDAServerConfigType::Celestia(celestia_config)) => {
                assert_eq!(
                    celestia_config,
                    DAServerCelestiaConfig {
                        namespace: Some("test_namespace".to_string()),
                        conn: Some("test_conn".to_string()),
                        auth_token: Some("test_token".to_string()),
                        max_segment_size: Some(2048),
                    }
                );
            }
            Ok(_) => {
                panic!("Expected Celestia Config");
            }
            Err(e) => {
                panic!("Error parsing Celestia Config: {}", e)
            }
        }

        let mut config: HashMap<String, String> = HashMap::new();
        config.insert("Param1".to_string(), "value1".to_string());
        config.insert("param2".to_string(), "Value2".to_string());

        match InternalDAServerConfigType::from_str(openda_config_str) {
            Ok(InternalDAServerConfigType::OpenDA(openda_config)) => {
                assert_eq!(
                    openda_config,
                    DAServerOpenDAConfig {
                        scheme: OpenDAScheme::GCS,
                        config,
                        max_segment_size: Some(2048),
                    }
                );
            }
            Ok(_) => {
                panic!("Expected OpenDA Config");
            }
            Err(e) => {
                panic!("Error parsing OpenDA Config: {}", e)
            }
        }

        if let Err(_) = InternalDAServerConfigType::from_str(invalid_config_str) {
        } else {
            panic!("Expected Error for invalid config");
        }
    }
}
