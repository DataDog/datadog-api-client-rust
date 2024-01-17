// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Type of aggregation performed in the monitor query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptionsAggregation {
    /// Group to break down the monitor on.
    #[serde(rename = "group_by")]
    pub group_by: Option<String>,
    /// Metric name used in the monitor.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Metric type used in the monitor.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl MonitorOptionsAggregation {
    pub fn new() -> MonitorOptionsAggregation {
        MonitorOptionsAggregation {
            group_by: None,
            metric: None,
            type_: None,
        }
    }
}
impl Default for MonitorOptionsAggregation {
    fn default() -> Self {
        Self::new()
    }
}
