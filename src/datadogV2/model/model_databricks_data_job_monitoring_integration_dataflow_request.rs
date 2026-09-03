// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Databricks Data Jobs Monitoring dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksDataJobMonitoringIntegrationDataflowRequest {
    /// Whether the Databricks dataflow is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Settings of the Databricks Data Jobs Monitoring dataflow. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings: Option<
        crate::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowSettingsRequest,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksDataJobMonitoringIntegrationDataflowRequest {
    pub fn new() -> DatabricksDataJobMonitoringIntegrationDataflowRequest {
        DatabricksDataJobMonitoringIntegrationDataflowRequest {
            enabled: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowSettingsRequest,
    ) -> Self {
        self.settings = Some(value);
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

impl Default for DatabricksDataJobMonitoringIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksDataJobMonitoringIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksDataJobMonitoringIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for DatabricksDataJobMonitoringIntegrationDataflowRequestVisitor {
            type Value = DatabricksDataJobMonitoringIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut settings: Option<crate::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowSettingsRequest> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DatabricksDataJobMonitoringIntegrationDataflowRequest {
                    enabled,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksDataJobMonitoringIntegrationDataflowRequestVisitor)
    }
}
