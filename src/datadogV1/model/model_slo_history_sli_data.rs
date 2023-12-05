// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value.
/// This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOHistorySLIData {
    /// A mapping of threshold `timeframe` to the remaining error budget.
    #[serde(rename = "error_budget_remaining")]
    pub error_budget_remaining: Option<std::collections::HashMap<String, f64>>,
    /// An array of error objects returned while querying the history data for the service level objective.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>>,
    /// For groups in a grouped SLO, this is the group name.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// For `monitor` based SLOs, this includes the aggregated history as arrays that include time series and uptime data where `0=monitor` is in `OK` state and `1=monitor` is in `alert` state.
    #[serde(rename = "history")]
    pub history: Option<Vec<Vec<f64>>>,
    /// For `monitor` based SLOs, this is the last modified timestamp in epoch seconds of the monitor.
    #[serde(rename = "monitor_modified")]
    pub monitor_modified: Option<i64>,
    /// For `monitor` based SLOs, this describes the type of monitor.
    #[serde(rename = "monitor_type")]
    pub monitor_type: Option<String>,
    /// For groups in a grouped SLO, this is the group name. For monitors in a multi-monitor SLO, this is the monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A mapping of threshold `timeframe` to number of accurate decimals, regardless of the from && to timestamp.
    #[serde(rename = "precision")]
    pub precision: Option<std::collections::HashMap<String, f64>>,
    /// For `monitor` based SLOs, when `true` this indicates that a replay is in progress to give an accurate uptime
    /// calculation.
    #[serde(rename = "preview")]
    pub preview: Option<bool>,
    /// The current SLI value of the SLO over the history window.
    #[serde(
        rename = "sli_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub sli_value: Option<Option<f64>>,
    /// The amount of decimal places the SLI value is accurate to for the given from `&&` to timestamp.
    #[serde(rename = "span_precision")]
    pub span_precision: Option<f64>,
    /// Use `sli_value` instead.
    #[deprecated]
    #[serde(rename = "uptime", default, with = "::serde_with::rust::double_option")]
    pub uptime: Option<Option<f64>>,
}

impl SLOHistorySLIData {
    pub fn new() -> SLOHistorySLIData {
        #[allow(deprecated)]
        SLOHistorySLIData {
            error_budget_remaining: None,
            errors: None,
            group: None,
            history: None,
            monitor_modified: None,
            monitor_type: None,
            name: None,
            precision: None,
            preview: None,
            sli_value: None,
            span_precision: None,
            uptime: None,
        }
    }
}