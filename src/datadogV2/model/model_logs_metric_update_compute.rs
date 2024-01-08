// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The compute rule to compute the log-based metric.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricUpdateCompute {
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
    /// Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
}

impl LogsMetricUpdateCompute {
    pub fn new() -> LogsMetricUpdateCompute {
        LogsMetricUpdateCompute {
            include_percentiles: None,
        }
    }
}
impl Default for LogsMetricUpdateCompute {
    fn default() -> Self {
        Self::new()
    }
}
