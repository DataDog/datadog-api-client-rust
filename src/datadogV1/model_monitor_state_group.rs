// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorStateGroup {
    /// Latest timestamp the monitor was in NO_DATA state.
    #[serde(rename = "last_nodata_ts", skip_serializing_if = "Option::is_none")]
    pub last_nodata_ts: i64,
    /// Latest timestamp of the notification sent for this monitor group.
    #[serde(rename = "last_notified_ts", skip_serializing_if = "Option::is_none")]
    pub last_notified_ts: i64,
    /// Latest timestamp the monitor group was resolved.
    #[serde(rename = "last_resolved_ts", skip_serializing_if = "Option::is_none")]
    pub last_resolved_ts: i64,
    /// Latest timestamp the monitor group triggered.
    #[serde(rename = "last_triggered_ts", skip_serializing_if = "Option::is_none")]
    pub last_triggered_ts: i64,
    /// The name of the monitor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The different states your monitor can be in.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: MonitorOverallStates,
}

