// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The log-based metric properties that will be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricUpdateAttributes {
    /// The compute rule to compute the log-based metric.
    #[serde(rename = "compute")]
    pub compute: Option<Box<crate::datadogV2::model::LogsMetricUpdateCompute>>,
    /// The log-based metric filter. Logs matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::LogsMetricFilter>>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::LogsMetricGroupBy>>,
}

impl LogsMetricUpdateAttributes {
    pub fn new() -> LogsMetricUpdateAttributes {
        LogsMetricUpdateAttributes {
            compute: None,
            filter: None,
            group_by: None,
        }
    }
}
impl Default for LogsMetricUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}
