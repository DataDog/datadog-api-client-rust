// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the task history logs dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
    /// How often task history logs are collected, in minutes. One of `5`, `15`, `30`, `60`, or `1440`. Defaults to `5`.
    #[serde(rename = "task_history_logs_interval_min")]
    pub task_history_logs_interval_min: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
        SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
            task_history_logs_interval_min: None,
            _unparsed: false,
        }
    }

    pub fn task_history_logs_interval_min(mut self, value: i64) -> Self {
        self.task_history_logs_interval_min = Some(value);
        self
    }
}

impl Default for SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequestVisitor {
            type Value = SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut task_history_logs_interval_min: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "task_history_logs_interval_min" => {
                            if v.is_null() {
                                continue;
                            }
                            task_history_logs_interval_min =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest {
                    task_history_logs_interval_min,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequestVisitor)
    }
}
