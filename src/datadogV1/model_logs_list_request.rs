// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListRequest {
    /// The log index on which the request is performed. For multi-index organizations,
the default is all live indexes. Historical indexes of rehydrated logs must be specified.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: String,
    /// Number of logs return in the response.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i32,
    /// The search query - following the log search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Time-ascending `asc` or time-descending `desc` results.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: LogsSort,
    /// Hash identifier of the first log to return in the list, available in a log `id` attribute.
This parameter is used for the pagination feature.

**Note**: This parameter is ignored if the corresponding log
is out of the scope of the specified time window.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: String,
    /// Timeframe to retrieve the log from.
    #[serde(rename = "time")]
    pub time: LogsListRequestTime,
}

