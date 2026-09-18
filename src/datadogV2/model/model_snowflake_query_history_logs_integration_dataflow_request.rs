// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-query logs that let you identify long-running, poorly performing, and expensive queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `false`; set to `true` to start collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Settings of the query history logs dataflow. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings: Option<
        crate::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest,
    >,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
    pub fn new() -> SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
        SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
            enabled: None,
            settings: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest,
    ) -> Self {
        self.settings = Some(value);
        self
    }
}

impl Default for SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeQueryHistoryLogsIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeQueryHistoryLogsIntegrationDataflowRequestVisitor {
            type Value = SnowflakeQueryHistoryLogsIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut settings: Option<crate::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest> = None;
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeQueryHistoryLogsIntegrationDataflowRequest {
                    enabled,
                    settings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeQueryHistoryLogsIntegrationDataflowRequestVisitor)
    }
}
