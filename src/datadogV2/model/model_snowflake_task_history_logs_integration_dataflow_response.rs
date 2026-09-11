// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Execution logs for your scheduled Snowflake tasks, covering start and end time, status, and any error message.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
    /// Whether Datadog collects this data.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Settings of the task history logs dataflow.
    #[serde(rename = "settings")]
    pub settings: Option<
        crate::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowSettingsResponse,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
    pub fn new() -> SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
        SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
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
        value: crate::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowSettingsResponse,
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

impl Default for SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeTaskHistoryLogsIntegrationDataflowResponseVisitor;
        impl<'a> Visitor<'a> for SnowflakeTaskHistoryLogsIntegrationDataflowResponseVisitor {
            type Value = SnowflakeTaskHistoryLogsIntegrationDataflowResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut settings: Option<crate::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowSettingsResponse> = None;
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

                let content = SnowflakeTaskHistoryLogsIntegrationDataflowResponse {
                    enabled,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeTaskHistoryLogsIntegrationDataflowResponseVisitor)
    }
}
