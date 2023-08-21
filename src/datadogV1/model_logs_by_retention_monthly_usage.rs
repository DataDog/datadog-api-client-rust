// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetentionMonthlyUsage {
    /// The month for the usage.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: String,
    /// Indexed logs usage for each active retention for the month.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Vec<LogsRetentionSumUsage>,
}

