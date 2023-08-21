// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorGroupSearchResult {
    /// The name of the group.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: String,
    /// The list of tags of the monitor group.
    #[serde(rename = "group_tags", skip_serializing_if = "Option::is_none")]
    pub group_tags: Vec<String>,
    /// Latest timestamp the monitor group was in NO_DATA state.
    #[serde(rename = "last_nodata_ts", skip_serializing_if = "Option::is_none")]
    pub last_nodata_ts: i64,
    /// Latest timestamp the monitor group triggered.
    #[serde(rename = "last_triggered_ts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub last_triggered_ts: Option<Int64>,
    /// The ID of the monitor.
    #[serde(rename = "monitor_id", skip_serializing_if = "Option::is_none")]
    pub monitor_id: i64,
    /// The name of the monitor.
    #[serde(rename = "monitor_name", skip_serializing_if = "Option::is_none")]
    pub monitor_name: String,
    /// The different states your monitor can be in.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: MonitorOverallStates,
}

