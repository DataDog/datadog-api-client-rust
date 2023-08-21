// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// Metric description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Name of the integration that sent the metric if applicable.
    #[serde(rename = "integration", skip_serializing_if = "Option::is_none")]
    pub integration: String,
    /// Per unit of the metric such as `second` in `bytes per second`.
    #[serde(rename = "per_unit", skip_serializing_if = "Option::is_none")]
    pub per_unit: String,
    /// A more human-readable and abbreviated version of the metric name.
    #[serde(rename = "short_name", skip_serializing_if = "Option::is_none")]
    pub short_name: String,
    /// StatsD flush interval of the metric in seconds if applicable.
    #[serde(rename = "statsd_interval", skip_serializing_if = "Option::is_none")]
    pub statsd_interval: i64,
    /// Metric type such as `gauge` or `rate`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
    /// Primary unit of the metric such as `byte` or `operation`.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: String,
}

