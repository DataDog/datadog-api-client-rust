// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric tag configuration attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationAttributes {
    /// A list of queryable aggregation combinations for a count, rate, or gauge metric.
    /// By default, count and rate metrics require the (time: sum, space: sum) aggregation and
    /// Gauge metrics require the (time: avg, space: avg) aggregation.
    /// Additional time & space combinations are also available:
    ///
    /// - time: avg, space: avg
    /// - time: avg, space: max
    /// - time: avg, space: min
    /// - time: avg, space: sum
    /// - time: count, space: sum
    /// - time: max, space: max
    /// - time: min, space: min
    /// - time: sum, space: avg
    /// - time: sum, space: sum
    ///
    /// Can only be applied to metrics that have a `metric_type` of `count`, `rate`, or `gauge`.
    #[serde(rename = "aggregations")]
    pub aggregations: Option<Vec<crate::datadogV2::model::MetricCustomAggregation>>,
    /// Timestamp when the tag configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// When set to true, the configuration will exclude the configured tags and include any other submitted tags.
    /// When set to false, the configuration will include the configured tags and exclude any other submitted tags.
    /// Defaults to false. Requires `tags` property.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
    /// Only present when the `metric_type` is `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
    /// The metric's type.
    #[serde(rename = "metric_type")]
    pub metric_type: Option<crate::datadogV2::model::MetricTagConfigurationMetricTypes>,
    /// Timestamp when the tag configuration was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// List of tag keys on which to group.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl MetricTagConfigurationAttributes {
    pub fn new() -> MetricTagConfigurationAttributes {
        MetricTagConfigurationAttributes {
            aggregations: None,
            created_at: None,
            exclude_tags_mode: None,
            include_percentiles: None,
            metric_type: None,
            modified_at: None,
            tags: None,
        }
    }
}
