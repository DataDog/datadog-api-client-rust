// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricEstimateAttributes {
    /// Estimate type based on the queried configuration. By default, `count_or_gauge` is returned. `distribution` is returned for distribution metrics without percentiles enabled. Lastly, `percentile` is returned if `filter[pct]=true` is queried with a distribution metric.
    #[serde(rename = "estimate_type", skip_serializing_if = "Option::is_none")]
    pub estimate_type: MetricEstimateType,
    /// Timestamp when the cardinality estimate was requested.
    #[serde(rename = "estimated_at", skip_serializing_if = "Option::is_none")]
    pub estimated_at: String,
    /// Estimated cardinality of the metric based on the queried configuration.
    #[serde(rename = "estimated_output_series", skip_serializing_if = "Option::is_none")]
    pub estimated_output_series: i64,
}

