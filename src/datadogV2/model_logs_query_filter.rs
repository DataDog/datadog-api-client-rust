// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsQueryFilter {
    /// The minimum time for the requested logs, supports date math and regular timestamps (milliseconds).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// For customers with multiple indexes, the indexes to search. Defaults to ['*'] which means all indexes.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Vec<String>,
    /// The search query - following the log search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Specifies storage type as indexes or online-archives
    #[serde(rename = "storage_tier", skip_serializing_if = "Option::is_none")]
    pub storage_tier: LogsStorageTier,
    /// The maximum time for the requested logs, supports date math and regular timestamps (milliseconds).
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: String,
}

