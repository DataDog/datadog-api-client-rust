// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A configuration file for an integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetConfigurationFileV2 {
    /// Hash of the configuration file as seen by the agent.
    #[serde(rename = "agent_hash")]
    pub agent_hash: Option<String>,
    /// The raw content of the configuration file.
    #[serde(rename = "file_content")]
    pub file_content: Option<String>,
    /// Path to the configuration file.
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    /// Name of the configuration file.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    /// Hash of the configuration file as applied by fleet management.
    #[serde(rename = "fleet_hash")]
    pub fleet_hash: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetConfigurationFileV2 {
    pub fn new() -> FleetConfigurationFileV2 {
        FleetConfigurationFileV2 {
            agent_hash: None,
            file_content: None,
            file_path: None,
            filename: None,
            fleet_hash: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_hash(mut self, value: String) -> Self {
        self.agent_hash = Some(value);
        self
    }

    pub fn file_content(mut self, value: String) -> Self {
        self.file_content = Some(value);
        self
    }

    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    pub fn filename(mut self, value: String) -> Self {
        self.filename = Some(value);
        self
    }

    pub fn fleet_hash(mut self, value: String) -> Self {
        self.fleet_hash = Some(value);
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

impl Default for FleetConfigurationFileV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetConfigurationFileV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetConfigurationFileV2Visitor;
        impl<'a> Visitor<'a> for FleetConfigurationFileV2Visitor {
            type Value = FleetConfigurationFileV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_hash: Option<String> = None;
                let mut file_content: Option<String> = None;
                let mut file_path: Option<String> = None;
                let mut filename: Option<String> = None;
                let mut fleet_hash: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_hash" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_content" => {
                            if v.is_null() {
                                continue;
                            }
                            file_content =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_path" => {
                            if v.is_null() {
                                continue;
                            }
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filename" => {
                            if v.is_null() {
                                continue;
                            }
                            filename = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fleet_hash" => {
                            if v.is_null() {
                                continue;
                            }
                            fleet_hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetConfigurationFileV2 {
                    agent_hash,
                    file_content,
                    file_path,
                    filename,
                    fleet_hash,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetConfigurationFileV2Visitor)
    }
}
