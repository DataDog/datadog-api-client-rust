// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Databricks Data Jobs Monitoring dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
    /// ID of the Datadog API key the global init script uses to submit data.
    #[serde(rename = "dd_api_key_id")]
    pub dd_api_key_id: Option<String>,
    /// Whether Datadog manages the global init script that installs the Agent on your Databricks clusters.
    #[serde(rename = "djm_global_init_script_enabled")]
    pub djm_global_init_script_enabled: Option<bool>,
    /// Whether GPU metrics are collected from your Databricks clusters. The Agent installed by the global init script performs the collection, so this requires the dataflow to be enabled with `djm_global_init_script_enabled` set to `true`.
    #[serde(rename = "script_gpum_enabled")]
    pub script_gpum_enabled: Option<bool>,
    /// Whether logs are collected from your Databricks clusters. The Agent installed by the global init script performs the collection, so this requires the dataflow to be enabled with `djm_global_init_script_enabled` set to `true`.
    #[serde(rename = "script_logs_enabled")]
    pub script_logs_enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
    pub fn new() -> DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
        DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
            dd_api_key_id: None,
            djm_global_init_script_enabled: None,
            script_gpum_enabled: None,
            script_logs_enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dd_api_key_id(mut self, value: String) -> Self {
        self.dd_api_key_id = Some(value);
        self
    }

    pub fn djm_global_init_script_enabled(mut self, value: bool) -> Self {
        self.djm_global_init_script_enabled = Some(value);
        self
    }

    pub fn script_gpum_enabled(mut self, value: bool) -> Self {
        self.script_gpum_enabled = Some(value);
        self
    }

    pub fn script_logs_enabled(mut self, value: bool) -> Self {
        self.script_logs_enabled = Some(value);
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

impl Default for DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksDataJobMonitoringIntegrationDataflowSettingsResponseVisitor;
        impl<'a> Visitor<'a> for DatabricksDataJobMonitoringIntegrationDataflowSettingsResponseVisitor {
            type Value = DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dd_api_key_id: Option<String> = None;
                let mut djm_global_init_script_enabled: Option<bool> = None;
                let mut script_gpum_enabled: Option<bool> = None;
                let mut script_logs_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dd_api_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            dd_api_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "djm_global_init_script_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            djm_global_init_script_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "script_gpum_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            script_gpum_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "script_logs_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            script_logs_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DatabricksDataJobMonitoringIntegrationDataflowSettingsResponse {
                    dd_api_key_id,
                    djm_global_init_script_enabled,
                    script_gpum_enabled,
                    script_logs_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(DatabricksDataJobMonitoringIntegrationDataflowSettingsResponseVisitor)
    }
}
