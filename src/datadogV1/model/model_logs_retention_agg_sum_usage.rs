// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing indexed logs usage aggregated across organizations and months for a retention period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsRetentionAggSumUsage {
    /// Total indexed logs for this retention period.
    #[serde(rename = "logs_indexed_logs_usage_agg_sum")]
    pub logs_indexed_logs_usage_agg_sum: Option<i64>,
    /// Live indexed logs for this retention period.
    #[serde(rename = "logs_live_indexed_logs_usage_agg_sum")]
    pub logs_live_indexed_logs_usage_agg_sum: Option<i64>,
    /// Rehydrated indexed logs for this retention period.
    #[serde(rename = "logs_rehydrated_indexed_logs_usage_agg_sum")]
    pub logs_rehydrated_indexed_logs_usage_agg_sum: Option<i64>,
    /// The retention period in days or "custom" for all custom retention periods.
    #[serde(rename = "retention")]
    pub retention: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsRetentionAggSumUsage {
    pub fn new() -> LogsRetentionAggSumUsage {
        LogsRetentionAggSumUsage {
            logs_indexed_logs_usage_agg_sum: None,
            logs_live_indexed_logs_usage_agg_sum: None,
            logs_rehydrated_indexed_logs_usage_agg_sum: None,
            retention: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn logs_indexed_logs_usage_agg_sum(mut self, value: i64) -> Self {
        self.logs_indexed_logs_usage_agg_sum = Some(value);
        self
    }

    pub fn logs_live_indexed_logs_usage_agg_sum(mut self, value: i64) -> Self {
        self.logs_live_indexed_logs_usage_agg_sum = Some(value);
        self
    }

    pub fn logs_rehydrated_indexed_logs_usage_agg_sum(mut self, value: i64) -> Self {
        self.logs_rehydrated_indexed_logs_usage_agg_sum = Some(value);
        self
    }

    pub fn retention(mut self, value: String) -> Self {
        self.retention = Some(value);
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

impl Default for LogsRetentionAggSumUsage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsRetentionAggSumUsage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsRetentionAggSumUsageVisitor;
        impl<'a> Visitor<'a> for LogsRetentionAggSumUsageVisitor {
            type Value = LogsRetentionAggSumUsage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut logs_indexed_logs_usage_agg_sum: Option<i64> = None;
                let mut logs_live_indexed_logs_usage_agg_sum: Option<i64> = None;
                let mut logs_rehydrated_indexed_logs_usage_agg_sum: Option<i64> = None;
                let mut retention: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "logs_indexed_logs_usage_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_logs_usage_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_live_indexed_logs_usage_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_live_indexed_logs_usage_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_rehydrated_indexed_logs_usage_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_rehydrated_indexed_logs_usage_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention" => {
                            if v.is_null() {
                                continue;
                            }
                            retention = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LogsRetentionAggSumUsage {
                    logs_indexed_logs_usage_agg_sum,
                    logs_live_indexed_logs_usage_agg_sum,
                    logs_rehydrated_indexed_logs_usage_agg_sum,
                    retention,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsRetentionAggSumUsageVisitor)
    }
}
