// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric estimate attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricEstimateAttributes {
    /// Estimate type based on the queried configuration. By default, `count_or_gauge` is returned. `distribution` is returned for distribution metrics without percentiles enabled. Lastly, `percentile` is returned if `filter[pct]=true` is queried with a distribution metric.
    #[serde(rename = "estimate_type")]
    pub estimate_type: Option<crate::datadogV2::model::MetricEstimateType>,
    /// Timestamp when the cardinality estimate was requested.
    #[serde(rename = "estimated_at")]
    pub estimated_at: Option<String>,
    /// Estimated cardinality of the metric based on the queried configuration.
    #[serde(rename = "estimated_output_series")]
    pub estimated_output_series: Option<i64>,
}

impl MetricEstimateAttributes {
    pub fn new() -> MetricEstimateAttributes {
        MetricEstimateAttributes {
            estimate_type: None,
            estimated_at: None,
            estimated_output_series: None,
        }
    }

    pub fn estimate_type(mut self, value: crate::datadogV2::model::MetricEstimateType) -> Self {
        self.estimate_type = Some(value);
        self
    }

    pub fn estimated_at(mut self, value: String) -> Self {
        self.estimated_at = Some(value);
        self
    }

    pub fn estimated_output_series(mut self, value: i64) -> Self {
        self.estimated_output_series = Some(value);
        self
    }
}

impl Default for MetricEstimateAttributes {
    fn default() -> Self {
        Self::new()
    }
}
