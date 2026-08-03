// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Detailed information about a single integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationDetailsV2 {
    /// Type of data collected, such as metrics or logs.
    #[serde(rename = "data_type")]
    pub data_type: Option<String>,
    /// Error messages if the integration has issues.
    #[serde(rename = "error_messages")]
    pub error_messages: Option<Vec<String>>,
    /// Initialization configuration (YAML format).
    #[serde(rename = "init_config")]
    pub init_config: Option<String>,
    /// Instance-specific configuration (YAML format).
    #[serde(rename = "instance_config")]
    pub instance_config: Option<String>,
    /// Whether this is a custom integration.
    #[serde(rename = "is_custom_check")]
    pub is_custom_check: Option<bool>,
    /// Whether this is a default integration instance.
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
    /// Whether this integration configuration is an init config.
    #[serde(rename = "is_init")]
    pub is_init: Option<bool>,
    /// Log collection configuration (YAML format).
    #[serde(rename = "log_config")]
    pub log_config: Option<String>,
    /// Name of the integration instance.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Number of pods running this integration. Absent from the response when the count is zero.
    #[serde(rename = "pod_count")]
    pub pod_count: Option<i64>,
    /// Index in the configuration file.
    #[serde(rename = "source_index")]
    pub source_index: Option<i64>,
    /// Path to the configuration file.
    #[serde(rename = "source_path")]
    pub source_path: Option<String>,
    /// Integration type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationDetailsV2 {
    pub fn new() -> FleetIntegrationDetailsV2 {
        FleetIntegrationDetailsV2 {
            data_type: None,
            error_messages: None,
            init_config: None,
            instance_config: None,
            is_custom_check: None,
            is_default: None,
            is_init: None,
            log_config: None,
            name: None,
            pod_count: None,
            source_index: None,
            source_path: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_type(mut self, value: String) -> Self {
        self.data_type = Some(value);
        self
    }

    pub fn error_messages(mut self, value: Vec<String>) -> Self {
        self.error_messages = Some(value);
        self
    }

    pub fn init_config(mut self, value: String) -> Self {
        self.init_config = Some(value);
        self
    }

    pub fn instance_config(mut self, value: String) -> Self {
        self.instance_config = Some(value);
        self
    }

    pub fn is_custom_check(mut self, value: bool) -> Self {
        self.is_custom_check = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn is_init(mut self, value: bool) -> Self {
        self.is_init = Some(value);
        self
    }

    pub fn log_config(mut self, value: String) -> Self {
        self.log_config = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pod_count(mut self, value: i64) -> Self {
        self.pod_count = Some(value);
        self
    }

    pub fn source_index(mut self, value: i64) -> Self {
        self.source_index = Some(value);
        self
    }

    pub fn source_path(mut self, value: String) -> Self {
        self.source_path = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for FleetIntegrationDetailsV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetIntegrationDetailsV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationDetailsV2Visitor;
        impl<'a> Visitor<'a> for FleetIntegrationDetailsV2Visitor {
            type Value = FleetIntegrationDetailsV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_type: Option<String> = None;
                let mut error_messages: Option<Vec<String>> = None;
                let mut init_config: Option<String> = None;
                let mut instance_config: Option<String> = None;
                let mut is_custom_check: Option<bool> = None;
                let mut is_default: Option<bool> = None;
                let mut is_init: Option<bool> = None;
                let mut log_config: Option<String> = None;
                let mut name: Option<String> = None;
                let mut pod_count: Option<i64> = None;
                let mut source_index: Option<i64> = None;
                let mut source_path: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_type" => {
                            if v.is_null() {
                                continue;
                            }
                            data_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
                            if v.is_null() {
                                continue;
                            }
                            error_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "init_config" => {
                            if v.is_null() {
                                continue;
                            }
                            init_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_config" => {
                            if v.is_null() {
                                continue;
                            }
                            instance_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_custom_check" => {
                            if v.is_null() {
                                continue;
                            }
                            is_custom_check =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_init" => {
                            if v.is_null() {
                                continue;
                            }
                            is_init = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log_config" => {
                            if v.is_null() {
                                continue;
                            }
                            log_config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pod_count" => {
                            if v.is_null() {
                                continue;
                            }
                            pod_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_index" => {
                            if v.is_null() {
                                continue;
                            }
                            source_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_path" => {
                            if v.is_null() {
                                continue;
                            }
                            source_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetIntegrationDetailsV2 {
                    data_type,
                    error_messages,
                    init_config,
                    instance_config,
                    is_custom_check,
                    is_default,
                    is_init,
                    log_config,
                    name,
                    pod_count,
                    source_index,
                    source_path,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationDetailsV2Visitor)
    }
}
