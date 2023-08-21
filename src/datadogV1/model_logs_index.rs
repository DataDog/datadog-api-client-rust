// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndex {
    /// The number of log events you can send in this index per day before you are rate-limited.
    #[serde(rename = "daily_limit", skip_serializing_if = "Option::is_none")]
    pub daily_limit: i64,
    /// An array of exclusion objects. The logs are tested against the query of each filter,
following the order of the array. Only the first matching active exclusion matters,
others (if any) are ignored.
    #[serde(rename = "exclusion_filters", skip_serializing_if = "Option::is_none")]
    pub exclusion_filters: Vec<LogsExclusion>,
    /// Filter for logs.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: LogsFilter,
    /// A boolean stating if the index is rate limited, meaning more logs than the daily limit have been sent.
Rate limit is reset every-day at 2pm UTC.
    #[serde(rename = "is_rate_limited", skip_serializing_if = "Option::is_none")]
    pub is_rate_limited: bool,
    /// The name of the index.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The number of days before logs are deleted from this index. Available values depend on
retention plans specified in your organization's contract/subscriptions.
    #[serde(rename = "num_retention_days", skip_serializing_if = "Option::is_none")]
    pub num_retention_days: i64,
}

