// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The compute rule to compute the log-based metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsMetricResponseCompute {
    /// The type of aggregation to use.
    #[serde(rename = "aggregation_type")]
    pub aggregation_type: Option<crate::datadogV2::model::LogsMetricResponseComputeAggregationType>,
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
    /// Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
    /// The path to the value the log-based metric will aggregate on (only used if the aggregation type is a "distribution").
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsMetricResponseCompute {
    pub fn new() -> LogsMetricResponseCompute {
        LogsMetricResponseCompute {
            aggregation_type: None,
            include_percentiles: None,
            path: None,
            _unparsed: false,
        }
    }

    pub fn aggregation_type(
        mut self,
        value: crate::datadogV2::model::LogsMetricResponseComputeAggregationType,
    ) -> Self {
        self.aggregation_type = Some(value);
        self
    }

    pub fn include_percentiles(mut self, value: bool) -> Self {
        self.include_percentiles = Some(value);
        self
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }
}

impl Default for LogsMetricResponseCompute {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsMetricResponseCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsMetricResponseComputeVisitor;
        impl<'a> Visitor<'a> for LogsMetricResponseComputeVisitor {
            type Value = LogsMetricResponseCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation_type: Option<
                    crate::datadogV2::model::LogsMetricResponseComputeAggregationType,
                > = None;
                let mut include_percentiles: Option<bool> = None;
                let mut path: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation_type" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation_type) = aggregation_type {
                                match _aggregation_type {
                                    crate::datadogV2::model::LogsMetricResponseComputeAggregationType::UnparsedObject(_aggregation_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "include_percentiles" => {
                            if v.is_null() {
                                continue;
                            }
                            include_percentiles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            if v.is_null() {
                                continue;
                            }
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsMetricResponseCompute {
                    aggregation_type,
                    include_percentiles,
                    path,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsMetricResponseComputeVisitor)
    }
}
