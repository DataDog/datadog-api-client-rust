// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricCompute {
    /// The type of aggregation to use.
    #[serde(rename = "aggregation_type")]
    pub aggregation_type: crate::datadogV2::model::SpansMetricComputeAggregationType,
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
    /// Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles", skip_serializing_if = "Option::is_none")]
    pub include_percentiles: Option<bool>,
    /// The path to the value the span-based metric will aggregate on (only used if the aggregation type is a "distribution").
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl SpansMetricCompute {
    /// The compute rule to compute the span-based metric.
    pub fn new(aggregation_type: crate::datadogV2::model::SpansMetricComputeAggregationType) -> SpansMetricCompute {
        SpansMetricCompute {
            aggregation_type: aggregation_type,
            include_percentiles: None,
            path: None,
        }
    }
}
