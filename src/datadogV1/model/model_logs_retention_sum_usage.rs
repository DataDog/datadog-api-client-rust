// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing indexed logs usage grouped by retention period and summed.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsRetentionSumUsage {
    /// Total indexed logs for this retention period.
    #[serde(rename = "logs_indexed_logs_usage_sum")]
    pub logs_indexed_logs_usage_sum: Option<i64>,
    /// Live indexed logs for this retention period.
    #[serde(rename = "logs_live_indexed_logs_usage_sum")]
    pub logs_live_indexed_logs_usage_sum: Option<i64>,
    /// Rehydrated indexed logs for this retention period.
    #[serde(rename = "logs_rehydrated_indexed_logs_usage_sum")]
    pub logs_rehydrated_indexed_logs_usage_sum: Option<i64>,
    /// The retention period in days or "custom" for all custom retention periods.
    #[serde(rename = "retention")]
    pub retention: Option<String>,
}

impl LogsRetentionSumUsage {
    pub fn new() -> LogsRetentionSumUsage {
        LogsRetentionSumUsage {
            logs_indexed_logs_usage_sum: None,
            logs_live_indexed_logs_usage_sum: None,
            logs_rehydrated_indexed_logs_usage_sum: None,
            retention: None,
        }
    }
}
impl Default for LogsRetentionSumUsage {
    fn default() -> Self {
        Self::new()
    }
}
