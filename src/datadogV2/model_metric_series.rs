// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSeries {
    /// If the type of the metric is rate or count, define the corresponding interval.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
    /// Metadata for the metric.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: MetricMetadata,
    /// The name of the timeseries.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Points relating to a metric. All points must be objects with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past.
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Vec<MetricPoint>,
    /// A list of resources to associate with this metric.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Vec<MetricResource>,
    /// The source type name.
    #[serde(rename = "source_type_name", skip_serializing_if = "Option::is_none")]
    pub source_type_name: String,
    /// A list of tags associated with the metric.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The type of metric. The available types are `0` (unspecified), `1` (count), `2` (rate), and `3` (gauge).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MetricIntakeType,
    /// The unit of point value.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: String,
}

