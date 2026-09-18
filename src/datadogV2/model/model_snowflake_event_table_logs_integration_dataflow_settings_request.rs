// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the event table dataflow. Each record type is collected independently so that you can control ingestion costs, and every record type is ingested into Datadog as logs tagged with its `record_type`. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
    /// Whether records with a `record_type` of `event` are collected. Defaults to `false`.
    #[serde(rename = "event_table_events_enabled")]
    pub event_table_events_enabled: Option<bool>,
    /// Whether records with a `record_type` of `log` are collected. Defaults to `false`.
    #[serde(rename = "event_table_logs_enabled")]
    pub event_table_logs_enabled: Option<bool>,
    /// How often event table records are collected, in minutes. One of `5`, `15`, `30`, `60`, or `1440`. Defaults to `5`.
    #[serde(rename = "event_table_logs_interval_min")]
    pub event_table_logs_interval_min: Option<i64>,
    /// Whether records with a `record_type` of `span_event` are collected. Defaults to `false`.
    #[serde(rename = "event_table_span_events_enabled")]
    pub event_table_span_events_enabled: Option<bool>,
    /// Whether records with a `record_type` of `span` are collected. Defaults to `false`.
    #[serde(rename = "event_table_spans_enabled")]
    pub event_table_spans_enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
        SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
            event_table_events_enabled: None,
            event_table_logs_enabled: None,
            event_table_logs_interval_min: None,
            event_table_span_events_enabled: None,
            event_table_spans_enabled: None,
            _unparsed: false,
        }
    }

    pub fn event_table_events_enabled(mut self, value: bool) -> Self {
        self.event_table_events_enabled = Some(value);
        self
    }

    pub fn event_table_logs_enabled(mut self, value: bool) -> Self {
        self.event_table_logs_enabled = Some(value);
        self
    }

    pub fn event_table_logs_interval_min(mut self, value: i64) -> Self {
        self.event_table_logs_interval_min = Some(value);
        self
    }

    pub fn event_table_span_events_enabled(mut self, value: bool) -> Self {
        self.event_table_span_events_enabled = Some(value);
        self
    }

    pub fn event_table_spans_enabled(mut self, value: bool) -> Self {
        self.event_table_spans_enabled = Some(value);
        self
    }
}

impl Default for SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeEventTableLogsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeEventTableLogsIntegrationDataflowSettingsRequestVisitor {
            type Value = SnowflakeEventTableLogsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_table_events_enabled: Option<bool> = None;
                let mut event_table_logs_enabled: Option<bool> = None;
                let mut event_table_logs_interval_min: Option<i64> = None;
                let mut event_table_span_events_enabled: Option<bool> = None;
                let mut event_table_spans_enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_table_events_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            event_table_events_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_table_logs_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            event_table_logs_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_table_logs_interval_min" => {
                            if v.is_null() {
                                continue;
                            }
                            event_table_logs_interval_min =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_table_span_events_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            event_table_span_events_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_table_spans_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            event_table_spans_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeEventTableLogsIntegrationDataflowSettingsRequest {
                    event_table_events_enabled,
                    event_table_logs_enabled,
                    event_table_logs_interval_min,
                    event_table_span_events_enabled,
                    event_table_spans_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SnowflakeEventTableLogsIntegrationDataflowSettingsRequestVisitor)
    }
}
