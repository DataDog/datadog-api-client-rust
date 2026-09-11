// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Jobs Monitoring, which collects performance, reliability, and cost data for your Databricks jobs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse {
    /// Whether Datadog collects this data. Set to `false` to stop collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Settings of the Data Jobs Monitoring dataflow.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowSettingsResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse {
    pub fn new() -> DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse {
        DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse {
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
        value: crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowSettingsResponse,
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

impl Default for DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponseVisitor;
        impl<'a> Visitor<'a>
            for DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponseVisitor
        {
            type Value = DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut settings: Option<crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowSettingsResponse> = None;
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

                let content =
                    DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse {
                        enabled,
                        settings,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponseVisitor,
        )
    }
}
