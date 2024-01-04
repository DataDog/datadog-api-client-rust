// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The compute rule to compute the span-based metric.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricCompute {
    /// The type of aggregation to use.
    #[serde(rename = "aggregation_type")]
    pub aggregation_type: crate::datadogV2::model::SpansMetricComputeAggregationType,
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
    /// Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
    /// The path to the value the span-based metric will aggregate on (only used if the aggregation type is a "distribution").
    #[serde(rename = "path")]
    pub path: Option<String>,
}

impl SpansMetricCompute {
    pub fn new(
        aggregation_type: crate::datadogV2::model::SpansMetricComputeAggregationType,
    ) -> SpansMetricCompute {
        SpansMetricCompute {
            aggregation_type,
            include_percentiles: None,
            path: None,
        }
    }
}
