// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryResponseData {
    /// The `from` timestamp in epoch seconds.
    #[serde(rename = "from_ts", skip_serializing_if = "Option::is_none")]
    pub from_ts: i64,
    /// For `metric` based SLOs where the query includes a group-by clause, this represents the list of grouping parameters.

This is not included in responses for `monitor` based SLOs.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<String>,
    /// For grouped SLOs, this represents SLI data for specific groups.

This is not included in the responses for `metric` based SLOs.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Vec<SLOHistoryMonitor>,
    /// For multi-monitor SLOs, this represents SLI data for specific monitors.

This is not included in the responses for `metric` based SLOs.
    #[serde(rename = "monitors", skip_serializing_if = "Option::is_none")]
    pub monitors: Vec<SLOHistoryMonitor>,
    /// An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value.
This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.
    #[serde(rename = "overall", skip_serializing_if = "Option::is_none")]
    pub overall: SLOHistorySLIData,
    /// A `metric` based SLO history response.

This is not included in responses for `monitor` based SLOs.
    #[serde(rename = "series")]
    pub series: SLOHistoryMetrics,
    /// mapping of string timeframe to the SLO threshold.
    #[serde(rename = "thresholds", skip_serializing_if = "Option::is_none")]
    pub thresholds: map[string]SLOThreshold,
    /// The `to` timestamp in epoch seconds.
    #[serde(rename = "to_ts", skip_serializing_if = "Option::is_none")]
    pub to_ts: i64,
    /// The type of the service level objective.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SLOType,
    /// A numeric representation of the type of the service level objective (`0` for
monitor, `1` for metric). Always included in service level objective responses.
Ignored in create/update requests.
    #[serde(rename = "type_id", skip_serializing_if = "Option::is_none")]
    pub type_id: SLOTypeNumeric,
}

