// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An array of service level objective objects.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryResponseData {
    /// The `from` timestamp in epoch seconds.
    #[serde(rename = "from_ts")]
    pub from_ts: Option<i64>,
    /// For `metric` based SLOs where the query includes a group-by clause, this represents the list of grouping parameters.
    ///
    /// This is not included in responses for `monitor` based SLOs.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// For grouped SLOs, this represents SLI data for specific groups.
    ///
    /// This is not included in the responses for `metric` based SLOs.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::datadogV1::model::SLOHistoryMonitor>>,
    /// For multi-monitor SLOs, this represents SLI data for specific monitors.
    ///
    /// This is not included in the responses for `metric` based SLOs.
    #[serde(rename = "monitors")]
    pub monitors: Option<Vec<crate::datadogV1::model::SLOHistoryMonitor>>,
    /// An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value.
    /// This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.
    #[serde(rename = "overall")]
    pub overall: Option<Box<crate::datadogV1::model::SLOHistorySLIData>>,
    /// A `metric` based SLO history response.
    ///
    /// This is not included in responses for `monitor` based SLOs.
    #[serde(rename = "series")]
    pub series: Option<Box<crate::datadogV1::model::SLOHistoryMetrics>>,
    /// mapping of string timeframe to the SLO threshold.
    #[serde(rename = "thresholds")]
    pub thresholds:
        Option<std::collections::BTreeMap<String, crate::datadogV1::model::SLOThreshold>>,
    /// The `to` timestamp in epoch seconds.
    #[serde(rename = "to_ts")]
    pub to_ts: Option<i64>,
    /// The type of the service level objective.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SLOType>,
    /// A numeric representation of the type of the service level objective (`0` for
    /// monitor, `1` for metric). Always included in service level objective responses.
    /// Ignored in create/update requests.
    #[serde(rename = "type_id")]
    pub type_id: Option<crate::datadogV1::model::SLOTypeNumeric>,
}

impl SLOHistoryResponseData {
    pub fn new() -> SLOHistoryResponseData {
        SLOHistoryResponseData {
            from_ts: None,
            group_by: None,
            groups: None,
            monitors: None,
            overall: None,
            series: None,
            thresholds: None,
            to_ts: None,
            type_: None,
            type_id: None,
        }
    }
}
impl Default for SLOHistoryResponseData {
    fn default() -> Self {
        Self::new()
    }
}
