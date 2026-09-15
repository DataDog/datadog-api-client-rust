// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the event table dataflow. Each record type is collected independently so that you can control ingestion costs, and every record type is ingested into Datadog as logs tagged with its `record_type`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
    /// Whether records with a `record_type` of `event` are collected.
    #[serde(rename = "event_table_events_enabled")]
    pub event_table_events_enabled: Option<bool>,
    /// Whether records with a `record_type` of `log` are collected.
    #[serde(rename = "event_table_logs_enabled")]
    pub event_table_logs_enabled: Option<bool>,
    /// How often event table records are collected, in minutes.
    #[serde(rename = "event_table_logs_interval_min")]
    pub event_table_logs_interval_min: Option<i64>,
    /// Whether records with a `record_type` of `span_event` are collected.
    #[serde(rename = "event_table_span_events_enabled")]
    pub event_table_span_events_enabled: Option<bool>,
    /// Whether records with a `record_type` of `span` are collected.
    #[serde(rename = "event_table_spans_enabled")]
    pub event_table_spans_enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
    pub fn new() -> SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
        SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
            event_table_events_enabled: None,
            event_table_logs_enabled: None,
            event_table_logs_interval_min: None,
            event_table_span_events_enabled: None,
            event_table_spans_enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeEventTableLogsIntegrationDataflowSettingsResponseVisitor;
        impl<'a> Visitor<'a> for SnowflakeEventTableLogsIntegrationDataflowSettingsResponseVisitor {
            type Value = SnowflakeEventTableLogsIntegrationDataflowSettingsResponse;

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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SnowflakeEventTableLogsIntegrationDataflowSettingsResponse {
                    event_table_events_enabled,
                    event_table_logs_enabled,
                    event_table_logs_interval_min,
                    event_table_span_events_enabled,
                    event_table_spans_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SnowflakeEventTableLogsIntegrationDataflowSettingsResponseVisitor)
    }
}
