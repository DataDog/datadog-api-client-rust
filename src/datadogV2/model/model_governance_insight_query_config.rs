// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query execution context that allows the frontend to execute insight queries directly.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceInsightQueryConfig {
    /// The chart type the frontend should use to render the insight.
    #[serde(rename = "chart_type")]
    pub chart_type: Option<String>,
    /// The window used for the previous value comparison; for example, `week` or `month`.
    #[serde(rename = "comparison_shift")]
    pub comparison_shift: String,
    /// The default value to display when no data is available.
    #[serde(rename = "default_value")]
    pub default_value: Option<i64>,
    /// Whether an increase in the value is good, bad, or neutral. One of `neutral`,
    /// `increase_better`, or `decrease_better`.
    #[serde(rename = "directionality")]
    pub directionality: Option<String>,
    /// The number of days the insight value is computed over.
    #[serde(rename = "effective_time_window_days")]
    pub effective_time_window_days: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceInsightQueryConfig {
    pub fn new(
        comparison_shift: String,
        effective_time_window_days: i64,
    ) -> GovernanceInsightQueryConfig {
        GovernanceInsightQueryConfig {
            chart_type: None,
            comparison_shift,
            default_value: None,
            directionality: None,
            effective_time_window_days,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn chart_type(mut self, value: String) -> Self {
        self.chart_type = Some(value);
        self
    }

    pub fn default_value(mut self, value: i64) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn directionality(mut self, value: String) -> Self {
        self.directionality = Some(value);
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

impl<'de> Deserialize<'de> for GovernanceInsightQueryConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceInsightQueryConfigVisitor;
        impl<'a> Visitor<'a> for GovernanceInsightQueryConfigVisitor {
            type Value = GovernanceInsightQueryConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chart_type: Option<String> = None;
                let mut comparison_shift: Option<String> = None;
                let mut default_value: Option<i64> = None;
                let mut directionality: Option<String> = None;
                let mut effective_time_window_days: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chart_type" => {
                            if v.is_null() {
                                continue;
                            }
                            chart_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "comparison_shift" => {
                            comparison_shift =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_value" => {
                            if v.is_null() {
                                continue;
                            }
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "directionality" => {
                            if v.is_null() {
                                continue;
                            }
                            directionality =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "effective_time_window_days" => {
                            effective_time_window_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let comparison_shift =
                    comparison_shift.ok_or_else(|| M::Error::missing_field("comparison_shift"))?;
                let effective_time_window_days = effective_time_window_days
                    .ok_or_else(|| M::Error::missing_field("effective_time_window_days"))?;

                let content = GovernanceInsightQueryConfig {
                    chart_type,
                    comparison_shift,
                    default_value,
                    directionality,
                    effective_time_window_days,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceInsightQueryConfigVisitor)
    }
}
