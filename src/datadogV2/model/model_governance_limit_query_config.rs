// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The query execution context used to visualize a limit and its usage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceLimitQueryConfig {
    /// The chart type used to visualize the limit and its usage.
    #[serde(rename = "chart_type")]
    pub chart_type: Option<String>,
    /// The time shift applied to compare current usage against a prior period.
    #[serde(rename = "comparison_shift")]
    pub comparison_shift: Option<String>,
    /// The default value used for the limit when no explicit value is configured.
    #[serde(rename = "default_value")]
    pub default_value: Option<i64>,
    /// The direction in which usage approaches the limit, indicating whether higher or lower values are closer to the limit.
    #[serde(rename = "directionality")]
    pub directionality: Option<String>,
    /// The number of days of data the limit query evaluates over.
    #[serde(rename = "effective_time_window_days")]
    pub effective_time_window_days: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceLimitQueryConfig {
    pub fn new() -> GovernanceLimitQueryConfig {
        GovernanceLimitQueryConfig {
            chart_type: None,
            comparison_shift: None,
            default_value: None,
            directionality: None,
            effective_time_window_days: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn chart_type(mut self, value: String) -> Self {
        self.chart_type = Some(value);
        self
    }

    pub fn comparison_shift(mut self, value: String) -> Self {
        self.comparison_shift = Some(value);
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

    pub fn effective_time_window_days(mut self, value: i64) -> Self {
        self.effective_time_window_days = Some(value);
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

impl Default for GovernanceLimitQueryConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GovernanceLimitQueryConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceLimitQueryConfigVisitor;
        impl<'a> Visitor<'a> for GovernanceLimitQueryConfigVisitor {
            type Value = GovernanceLimitQueryConfig;

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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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

                let content = GovernanceLimitQueryConfig {
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

        deserializer.deserialize_any(GovernanceLimitQueryConfigVisitor)
    }
}
