// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultShort {
    /// Last time the browser test was performed.
    #[serde(rename = "check_time", skip_serializing_if = "Option::is_none")]
    pub check_time: f64,
    /// Location from which the Browser test was performed.
    #[serde(rename = "probe_dc", skip_serializing_if = "Option::is_none")]
    pub probe_dc: String,
    /// Object with the result of the last browser test run.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: SyntheticsBrowserTestResultShortResult,
    /// ID of the browser test result.
    #[serde(rename = "result_id", skip_serializing_if = "Option::is_none")]
    pub result_id: String,
    /// The status of your Synthetic monitor.
* `O` for not triggered
* `1` for triggered
* `2` for no data
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: SyntheticsTestMonitorStatus,
}

