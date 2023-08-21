// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryMetadata {
    /// Aggregation type.
    #[serde(rename = "aggr", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub aggr: Option<String>,
    /// Display name of the metric.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// End of the time window, milliseconds since Unix epoch.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: i64,
    /// Metric expression.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: String,
    /// Number of milliseconds between data samples.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
    /// Number of data samples.
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: i64,
    /// Metric name.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// List of points of the time series in milliseconds.
    #[serde(rename = "pointlist", skip_serializing_if = "Option::is_none")]
    pub pointlist: Vec<Vec<*f64>>,
    /// The index of the series' query within the request.
    #[serde(rename = "query_index", skip_serializing_if = "Option::is_none")]
    pub query_index: i64,
    /// Metric scope, comma separated list of tags.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
    /// Start of the time window, milliseconds since Unix epoch.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: i64,
    /// Unique tags identifying this series.
    #[serde(rename = "tag_set", skip_serializing_if = "Option::is_none")]
    pub tag_set: Vec<String>,
    /// Detailed information about the metric unit.
The first element describes the "primary unit" (for example, `bytes` in `bytes per second`).
The second element describes the "per unit" (for example, `second` in `bytes per second`).
If the second element is not present, the API returns null.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Vec<MetricsQueryUnit>,
}

