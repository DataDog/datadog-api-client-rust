// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric tag configuration to be created.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationCreateAttributes {
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
    /// When set to true, the configuration will exclude the configured tags and include any other submitted tags.
    /// When set to false, the configuration will include the configured tags and exclude any other submitted tags.
    /// Defaults to false. Requires `tags` property.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Toggle to include/exclude percentiles for a distribution metric.
    /// Defaults to false. Can only be applied to metrics that have a `metric_type` of `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
    /// The metric's type.
    #[serde(rename = "metric_type")]
    pub metric_type: crate::datadogV2::model::MetricTagConfigurationMetricTypes,
    /// A list of tag keys that will be queryable for your metric.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl MetricTagConfigurationCreateAttributes {
    pub fn new(
        metric_type: crate::datadogV2::model::MetricTagConfigurationMetricTypes,
        tags: Vec<String>,
    ) -> MetricTagConfigurationCreateAttributes {
        MetricTagConfigurationCreateAttributes {
            aggregations: None,
            exclude_tags_mode: None,
            include_percentiles: None,
            metric_type,
            tags,
        }
    }

    pub fn with_aggregations(
        &mut self,
        value: Vec<crate::datadogV2::model::MetricCustomAggregation>,
    ) -> &mut Self {
        self.aggregations = Some(value);
        self
    }

    pub fn with_exclude_tags_mode(&mut self, value: bool) -> &mut Self {
        self.exclude_tags_mode = Some(value);
        self
    }

    pub fn with_include_percentiles(&mut self, value: bool) -> &mut Self {
        self.include_percentiles = Some(value);
        self
    }
}
