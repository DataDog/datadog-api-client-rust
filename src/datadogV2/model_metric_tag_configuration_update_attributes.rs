// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationUpdateAttributes {
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
    /// Toggle to include/exclude percentiles for a distribution metric.
Defaults to false. Can only be applied to metrics that have a `metric_type` of `distribution`.
    #[serde(rename = "include_percentiles", skip_serializing_if = "Option::is_none")]
    pub include_percentiles: bool,
    /// A list of tag keys that will be queryable for your metric.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

