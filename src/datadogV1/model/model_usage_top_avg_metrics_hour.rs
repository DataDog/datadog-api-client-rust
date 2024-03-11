// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of hourly recorded custom metrics for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageTopAvgMetricsHour {
    /// Average number of timeseries per hour in which the metric occurs.
    #[serde(rename = "avg_metric_hour")]
    pub avg_metric_hour: Option<i64>,
    /// Maximum number of timeseries per hour in which the metric occurs.
    #[serde(rename = "max_metric_hour")]
    pub max_metric_hour: Option<i64>,
    /// Contains the metric category.
    #[serde(rename = "metric_category")]
    pub metric_category: Option<crate::datadogV1::model::UsageMetricCategory>,
    /// Contains the custom metric name.
    #[serde(rename = "metric_name")]
    pub metric_name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageTopAvgMetricsHour {
    pub fn new() -> UsageTopAvgMetricsHour {
        UsageTopAvgMetricsHour {
            avg_metric_hour: None,
            max_metric_hour: None,
            metric_category: None,
            metric_name: None,
            _unparsed: false,
        }
    }

    pub fn avg_metric_hour(&mut self, value: i64) -> &mut Self {
        self.avg_metric_hour = Some(value);
        self
    }

    pub fn max_metric_hour(&mut self, value: i64) -> &mut Self {
        self.max_metric_hour = Some(value);
        self
    }

    pub fn metric_category(
        &mut self,
        value: crate::datadogV1::model::UsageMetricCategory,
    ) -> &mut Self {
        self.metric_category = Some(value);
        self
    }

    pub fn metric_name(&mut self, value: String) -> &mut Self {
        self.metric_name = Some(value);
        self
    }
}

impl Default for UsageTopAvgMetricsHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageTopAvgMetricsHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageTopAvgMetricsHourVisitor;
        impl<'a> Visitor<'a> for UsageTopAvgMetricsHourVisitor {
            type Value = UsageTopAvgMetricsHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_metric_hour: Option<i64> = None;
                let mut max_metric_hour: Option<i64> = None;
                let mut metric_category: Option<crate::datadogV1::model::UsageMetricCategory> =
                    None;
                let mut metric_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_metric_hour" => {
                            if v.is_null() {
                                continue;
                            }
                            avg_metric_hour =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_metric_hour" => {
                            if v.is_null() {
                                continue;
                            }
                            max_metric_hour =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_category" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_category =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric_category) = metric_category {
                                match _metric_category {
                                    crate::datadogV1::model::UsageMetricCategory::UnparsedObject(_metric_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "metric_name" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageTopAvgMetricsHour {
                    avg_metric_hour,
                    max_metric_hour,
                    metric_category,
                    metric_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageTopAvgMetricsHourVisitor)
    }
}
