// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing all metric names returned and their associated metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    pub fn aggr(&mut self, value: Option<String>) -> &mut Self {
        self.aggr = Some(value);
        self
    }

    pub fn display_name(&mut self, value: String) -> &mut Self {
        self.display_name = Some(value);
        self
    }

    pub fn end(&mut self, value: i64) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn expression(&mut self, value: String) -> &mut Self {
        self.expression = Some(value);
        self
    }

    pub fn interval(&mut self, value: i64) -> &mut Self {
        self.interval = Some(value);
        self
    }

    pub fn length(&mut self, value: i64) -> &mut Self {
        self.length = Some(value);
        self
    }

    pub fn metric(&mut self, value: String) -> &mut Self {
        self.metric = Some(value);
        self
    }

    pub fn pointlist(&mut self, value: Vec<Vec<Option<f64>>>) -> &mut Self {
        self.pointlist = Some(value);
        self
    }

    pub fn query_index(&mut self, value: i64) -> &mut Self {
        self.query_index = Some(value);
        self
    }

    pub fn scope(&mut self, value: String) -> &mut Self {
        self.scope = Some(value);
        self
    }

    pub fn start(&mut self, value: i64) -> &mut Self {
        self.start = Some(value);
        self
    }

    pub fn tag_set(&mut self, value: Vec<String>) -> &mut Self {
        self.tag_set = Some(value);
        self
    }

    pub fn unit(
        &mut self,
        value: Vec<Option<crate::datadogV1::model::MetricsQueryUnit>>,
    ) -> &mut Self {
        self.unit = Some(value);
        self
    }
}

impl Default for MetricsQueryMetadata {
    fn default() -> Self {
        Self::new()
    }
}
