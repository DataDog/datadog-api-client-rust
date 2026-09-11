// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the query history logs dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
    /// Whether query logs are joined with Snowflake access history, which adds the objects each query read and wrote so you can follow how data is used and where it came from. Defaults to `false`.
    #[serde(rename = "join_query_history_with_access_history_enabled")]
    pub join_query_history_with_access_history_enabled: Option<bool>,
    /// How often query history logs are collected, in minutes. One of `5`, `15`, `30`, `60`, `720`, or `1440`. Defaults to `5`.
    #[serde(rename = "query_history_logs_interval_min")]
    pub query_history_logs_interval_min: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
        SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
            join_query_history_with_access_history_enabled: None,
            query_history_logs_interval_min: None,
            _unparsed: false,
        }
    }

    pub fn join_query_history_with_access_history_enabled(mut self, value: bool) -> Self {
        self.join_query_history_with_access_history_enabled = Some(value);
        self
    }

    pub fn query_history_logs_interval_min(mut self, value: i64) -> Self {
        self.query_history_logs_interval_min = Some(value);
        self
    }
}

impl Default for SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequestVisitor {
            type Value = SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut join_query_history_with_access_history_enabled: Option<bool> = None;
                let mut query_history_logs_interval_min: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "join_query_history_with_access_history_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            join_query_history_with_access_history_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_history_logs_interval_min" => {
                            if v.is_null() {
                                continue;
                            }
                            query_history_logs_interval_min =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest {
                    join_query_history_with_access_history_enabled,
                    query_history_logs_interval_min,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequestVisitor)
    }
}
