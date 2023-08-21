// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndexUpdateRequest {
    /// The number of log events you can send in this index per day before you are rate-limited.
    #[serde(rename = "daily_limit", skip_serializing_if = "Option::is_none")]
    pub daily_limit: i64,
    /// If true, sets the `daily_limit` value to null and the index is not limited on a daily basis (any
specified `daily_limit` value in the request is ignored). If false or omitted, the index's current
`daily_limit` is maintained.
    #[serde(rename = "disable_daily_limit", skip_serializing_if = "Option::is_none")]
    pub disable_daily_limit: bool,
    /// An array of exclusion objects. The logs are tested against the query of each filter,
following the order of the array. Only the first matching active exclusion matters,
others (if any) are ignored.
    #[serde(rename = "exclusion_filters", skip_serializing_if = "Option::is_none")]
    pub exclusion_filters: Vec<LogsExclusion>,
    /// Filter for logs.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: LogsFilter,
    /// The number of days before logs are deleted from this index. Available values depend on
retention plans specified in your organization's contract/subscriptions.

**Note:** Changing the retention for an index adjusts the length of retention for all logs
already in this index. It may also affect billing.
    #[serde(rename = "num_retention_days", skip_serializing_if = "Option::is_none")]
    pub num_retention_days: i64,
}

