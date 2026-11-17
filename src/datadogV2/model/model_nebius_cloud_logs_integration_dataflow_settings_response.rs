// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the logs dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudLogsIntegrationDataflowSettingsResponse {
    /// Nebius log integrations collected, such as `nebius-cloud-compute` and `default`. Absent when no service is enabled.
    #[serde(rename = "enabled_services")]
    pub enabled_services: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudLogsIntegrationDataflowSettingsResponse {
    pub fn new() -> NebiusCloudLogsIntegrationDataflowSettingsResponse {
        NebiusCloudLogsIntegrationDataflowSettingsResponse {
            enabled_services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled_services(mut self, value: Vec<String>) -> Self {
        self.enabled_services = Some(value);
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

impl Default for NebiusCloudLogsIntegrationDataflowSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudLogsIntegrationDataflowSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudLogsIntegrationDataflowSettingsResponseVisitor;
        impl<'a> Visitor<'a> for NebiusCloudLogsIntegrationDataflowSettingsResponseVisitor {
            type Value = NebiusCloudLogsIntegrationDataflowSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled_services: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled_services" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NebiusCloudLogsIntegrationDataflowSettingsResponse {
                    enabled_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudLogsIntegrationDataflowSettingsResponseVisitor)
    }
}
