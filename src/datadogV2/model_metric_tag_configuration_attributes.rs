// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationAttributes {
    /// A list of queryable aggregation combinations for a count, rate, or gauge metric.
By default, count and rate metrics require the (time: sum, space: sum) aggregation and
Gauge metrics require the (time: avg, space: avg) aggregation.
Additional time & space combinations are also available:

- time: avg, space: avg
- time: avg, space: max
- time: avg, space: min
- time: avg, space: sum
- time: count, space: sum
- time: max, space: max
- time: min, space: min
- time: sum, space: avg
- time: sum, space: sum

Can only be applied to metrics that have a `metric_type` of `count`, `rate`, or `gauge`.
    #[serde(rename = "aggregations", skip_serializing_if = "Option::is_none")]
    pub aggregations: Vec<MetricCustomAggregation>,
    /// Timestamp when the tag configuration was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
Only present when the `metric_type` is `distribution`.
    #[serde(rename = "include_percentiles", skip_serializing_if = "Option::is_none")]
    pub include_percentiles: bool,
    /// The metric's type.
    #[serde(rename = "metric_type", skip_serializing_if = "Option::is_none")]
    pub metric_type: MetricTagConfigurationMetricTypes,
    /// Timestamp when the tag configuration was last modified.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// List of tag keys on which to group.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

