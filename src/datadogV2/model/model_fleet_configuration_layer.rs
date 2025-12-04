// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration information organized by layers.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetConfigurationLayer {
    /// The final compiled configuration.
    #[serde(rename = "compiled_configuration")]
    pub compiled_configuration: Option<String>,
    /// Configuration from environment variables.
    #[serde(rename = "env_configuration")]
    pub env_configuration: Option<String>,
    /// Configuration from files.
    #[serde(rename = "file_configuration")]
    pub file_configuration: Option<String>,
    /// Parsed configuration output.
    #[serde(rename = "parsed_configuration")]
    pub parsed_configuration: Option<String>,
    /// Remote configuration settings.
    #[serde(rename = "remote_configuration")]
    pub remote_configuration: Option<String>,
    /// Runtime configuration.
    #[serde(rename = "runtime_configuration")]
    pub runtime_configuration: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetConfigurationLayer {
    pub fn new() -> FleetConfigurationLayer {
        FleetConfigurationLayer {
            compiled_configuration: None,
            env_configuration: None,
            file_configuration: None,
            parsed_configuration: None,
            remote_configuration: None,
            runtime_configuration: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compiled_configuration(mut self, value: String) -> Self {
        self.compiled_configuration = Some(value);
        self
    }

    pub fn env_configuration(mut self, value: String) -> Self {
        self.env_configuration = Some(value);
        self
    }

    pub fn file_configuration(mut self, value: String) -> Self {
        self.file_configuration = Some(value);
        self
    }

    pub fn parsed_configuration(mut self, value: String) -> Self {
        self.parsed_configuration = Some(value);
        self
    }

    pub fn remote_configuration(mut self, value: String) -> Self {
        self.remote_configuration = Some(value);
        self
    }

    pub fn runtime_configuration(mut self, value: String) -> Self {
        self.runtime_configuration = Some(value);
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

impl Default for FleetConfigurationLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetConfigurationLayer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetConfigurationLayerVisitor;
        impl<'a> Visitor<'a> for FleetConfigurationLayerVisitor {
            type Value = FleetConfigurationLayer;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compiled_configuration: Option<String> = None;
                let mut env_configuration: Option<String> = None;
                let mut file_configuration: Option<String> = None;
                let mut parsed_configuration: Option<String> = None;
                let mut remote_configuration: Option<String> = None;
                let mut runtime_configuration: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compiled_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            compiled_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            env_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            file_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parsed_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            parsed_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runtime_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            runtime_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetConfigurationLayer {
                    compiled_configuration,
                    env_configuration,
                    file_configuration,
                    parsed_configuration,
                    remote_configuration,
                    runtime_configuration,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetConfigurationLayerVisitor)
    }
}
