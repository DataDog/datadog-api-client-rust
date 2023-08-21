// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistorySLIData {
    /// A mapping of threshold `timeframe` to the remaining error budget.
    #[serde(rename = "error_budget_remaining", skip_serializing_if = "Option::is_none")]
    pub error_budget_remaining: map[string]f64,
    /// An array of error objects returned while querying the history data for the service level objective.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Vec<SLOHistoryResponseErrorWithType>,
    /// For groups in a grouped SLO, this is the group name.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: String,
    /// For `monitor` based SLOs, this includes the aggregated history as arrays that include time series and uptime data where `0=monitor` is in `OK` state and `1=monitor` is in `alert` state.
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Vec<Vec<f64>>,
    /// For `monitor` based SLOs, this is the last modified timestamp in epoch seconds of the monitor.
    #[serde(rename = "monitor_modified", skip_serializing_if = "Option::is_none")]
    pub monitor_modified: i64,
    /// For `monitor` based SLOs, this describes the type of monitor.
    #[serde(rename = "monitor_type", skip_serializing_if = "Option::is_none")]
    pub monitor_type: String,
    /// For groups in a grouped SLO, this is the group name. For monitors in a multi-monitor SLO, this is the monitor name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// A mapping of threshold `timeframe` to number of accurate decimals, regardless of the from && to timestamp.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: map[string]f64,
    /// For `monitor` based SLOs, when `true` this indicates that a replay is in progress to give an accurate uptime
calculation.
    #[serde(rename = "preview", skip_serializing_if = "Option::is_none")]
    pub preview: bool,
    /// The current SLI value of the SLO over the history window.
    #[serde(rename = "sli_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub sli_value: Option<Float64>,
    /// The amount of decimal places the SLI value is accurate to for the given from `&&` to timestamp.
    #[serde(rename = "span_precision", skip_serializing_if = "Option::is_none")]
    pub span_precision: f64,
    /// Use `sli_value` instead.
    #[serde(rename = "uptime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub uptime: Option<Float64>,
}

