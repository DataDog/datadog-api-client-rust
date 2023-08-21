// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsRetentionAggSumUsage {
    /// Total indexed logs for this retention period.
    #[serde(rename = "logs_indexed_logs_usage_agg_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_logs_usage_agg_sum: i64,
    /// Live indexed logs for this retention period.
    #[serde(rename = "logs_live_indexed_logs_usage_agg_sum", skip_serializing_if = "Option::is_none")]
    pub logs_live_indexed_logs_usage_agg_sum: i64,
    /// Rehydrated indexed logs for this retention period.
    #[serde(rename = "logs_rehydrated_indexed_logs_usage_agg_sum", skip_serializing_if = "Option::is_none")]
    pub logs_rehydrated_indexed_logs_usage_agg_sum: i64,
    /// The retention period in days or "custom" for all custom retention periods.
    #[serde(rename = "retention", skip_serializing_if = "Option::is_none")]
    pub retention: String,
}

