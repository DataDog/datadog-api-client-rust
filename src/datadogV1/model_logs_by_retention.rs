// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetention {
    /// Indexed logs usage summary for each organization for each retention period with usage.
    #[serde(rename = "orgs", skip_serializing_if = "Option::is_none")]
    pub orgs: LogsByRetentionOrgs,
    /// Aggregated index logs usage for each retention period with usage.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Vec<LogsRetentionAggSumUsage>,
    /// Object containing a summary of indexed logs usage by retention period for a single month.
    #[serde(rename = "usage_by_month", skip_serializing_if = "Option::is_none")]
    pub usage_by_month: LogsByRetentionMonthlyUsage,
}

