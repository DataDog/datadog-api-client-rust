// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing all metric names returned and their associated metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricsQueryMetadata {
    /// Aggregation type.
    #[serde(rename = "aggr", default, with = "::serde_with::rust::double_option")]
    pub aggr: Option<Option<String>>,
    /// Display name of the metric.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// End of the time window, milliseconds since Unix epoch.
    #[serde(rename = "end")]
    pub end: Option<i64>,
    /// Metric expression.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// Number of milliseconds between data samples.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Number of data samples.
    #[serde(rename = "length")]
    pub length: Option<i64>,
    /// Metric name.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// List of points of the time series in milliseconds.
    #[serde(rename = "pointlist")]
    pub pointlist: Option<Vec<Vec<Option<f64>>>>,
    /// The index of the series' query within the request.
    #[serde(rename = "query_index")]
    pub query_index: Option<i64>,
    /// Metric scope, comma separated list of tags.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// Start of the time window, milliseconds since Unix epoch.
    #[serde(rename = "start")]
    pub start: Option<i64>,
    /// Unique tags identifying this series.
    #[serde(rename = "tag_set")]
    pub tag_set: Option<Vec<String>>,
    /// Detailed information about the metric unit.
    /// The first element describes the "primary unit" (for example, `bytes` in `bytes per second`).
    /// The second element describes the "per unit" (for example, `second` in `bytes per second`).
    /// If the second element is not present, the API returns null.
    #[serde(rename = "unit")]
    pub unit: Option<Vec<Option<crate::datadogV1::model::MetricsQueryUnit>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricsQueryMetadata {
    pub fn new() -> MetricsQueryMetadata {
        MetricsQueryMetadata {
            aggr: None,
            display_name: None,
            end: None,
            expression: None,
            interval: None,
            length: None,
            metric: None,
            pointlist: None,
            query_index: None,
            scope: None,
            start: None,
            tag_set: None,
            unit: None,
            _unparsed: false,
        }
    }

    pub fn aggr(mut self, value: Option<String>) -> Self {
        self.aggr = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn expression(mut self, value: String) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn length(mut self, value: i64) -> Self {
        self.length = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn pointlist(mut self, value: Vec<Vec<Option<f64>>>) -> Self {
        self.pointlist = Some(value);
        self
    }

    pub fn query_index(mut self, value: i64) -> Self {
        self.query_index = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn tag_set(mut self, value: Vec<String>) -> Self {
        self.tag_set = Some(value);
        self
    }

    pub fn unit(mut self, value: Vec<Option<crate::datadogV1::model::MetricsQueryUnit>>) -> Self {
        self.unit = Some(value);
        self
    }
}

impl Default for MetricsQueryMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricsQueryMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricsQueryMetadataVisitor;
        impl<'a> Visitor<'a> for MetricsQueryMetadataVisitor {
            type Value = MetricsQueryMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggr: Option<Option<String>> = None;
                let mut display_name: Option<String> = None;
                let mut end: Option<i64> = None;
                let mut expression: Option<String> = None;
                let mut interval: Option<i64> = None;
                let mut length: Option<i64> = None;
                let mut metric: Option<String> = None;
                let mut pointlist: Option<Vec<Vec<Option<f64>>>> = None;
                let mut query_index: Option<i64> = None;
                let mut scope: Option<String> = None;
                let mut start: Option<i64> = None;
                let mut tag_set: Option<Vec<String>> = None;
                let mut unit: Option<Vec<Option<crate::datadogV1::model::MetricsQueryUnit>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggr" => {
                            aggr = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            if v.is_null() {
                                continue;
                            }
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "length" => {
                            if v.is_null() {
                                continue;
                            }
                            length = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pointlist" => {
                            if v.is_null() {
                                continue;
                            }
                            pointlist = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_index" => {
                            if v.is_null() {
                                continue;
                            }
                            query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_set" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_set = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricsQueryMetadata {
                    aggr,
                    display_name,
                    end,
                    expression,
                    interval,
                    length,
                    metric,
                    pointlist,
                    query_index,
                    scope,
                    start,
                    tag_set,
                    unit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricsQueryMetadataVisitor)
    }
}
