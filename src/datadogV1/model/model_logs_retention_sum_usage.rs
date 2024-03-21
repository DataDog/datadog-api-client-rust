// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing indexed logs usage grouped by retention period and summed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsRetentionSumUsage {
    /// Total indexed logs for this retention period.
    #[serde(rename = "logs_indexed_logs_usage_sum")]
    pub logs_indexed_logs_usage_sum: Option<i64>,
    /// Live indexed logs for this retention period.
    #[serde(rename = "logs_live_indexed_logs_usage_sum")]
    pub logs_live_indexed_logs_usage_sum: Option<i64>,
    /// Rehydrated indexed logs for this retention period.
    #[serde(rename = "logs_rehydrated_indexed_logs_usage_sum")]
    pub logs_rehydrated_indexed_logs_usage_sum: Option<i64>,
    /// The retention period in days or "custom" for all custom retention periods.
    #[serde(rename = "retention")]
    pub retention: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsRetentionSumUsage {
    pub fn new() -> LogsRetentionSumUsage {
        LogsRetentionSumUsage {
            logs_indexed_logs_usage_sum: None,
            logs_live_indexed_logs_usage_sum: None,
            logs_rehydrated_indexed_logs_usage_sum: None,
            retention: None,
            _unparsed: false,
        }
    }

    pub fn logs_indexed_logs_usage_sum(mut self, value: i64) -> Self {
        self.logs_indexed_logs_usage_sum = Some(value);
        self
    }

    pub fn logs_live_indexed_logs_usage_sum(mut self, value: i64) -> Self {
        self.logs_live_indexed_logs_usage_sum = Some(value);
        self
    }

    pub fn logs_rehydrated_indexed_logs_usage_sum(mut self, value: i64) -> Self {
        self.logs_rehydrated_indexed_logs_usage_sum = Some(value);
        self
    }

    pub fn retention(mut self, value: String) -> Self {
        self.retention = Some(value);
        self
    }
}

impl Default for LogsRetentionSumUsage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsRetentionSumUsage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsRetentionSumUsageVisitor;
        impl<'a> Visitor<'a> for LogsRetentionSumUsageVisitor {
            type Value = LogsRetentionSumUsage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut logs_indexed_logs_usage_sum: Option<i64> = None;
                let mut logs_live_indexed_logs_usage_sum: Option<i64> = None;
                let mut logs_rehydrated_indexed_logs_usage_sum: Option<i64> = None;
                let mut retention: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "logs_indexed_logs_usage_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_logs_usage_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_live_indexed_logs_usage_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_live_indexed_logs_usage_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_rehydrated_indexed_logs_usage_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_rehydrated_indexed_logs_usage_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention" => {
                            if v.is_null() {
                                continue;
                            }
                            retention = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsRetentionSumUsage {
                    logs_indexed_logs_usage_sum,
                    logs_live_indexed_logs_usage_sum,
                    logs_rehydrated_indexed_logs_usage_sum,
                    retention,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsRetentionSumUsageVisitor)
    }
}
