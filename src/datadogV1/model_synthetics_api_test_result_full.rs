// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultFull {
    /// Object describing the API test configuration.
    #[serde(rename = "check")]
    pub check: SyntheticsAPITestResultFullCheck,
    /// When the API test was conducted.
    #[serde(rename = "check_time", skip_serializing_if = "Option::is_none")]
    pub check_time: f64,
    /// Version of the API test used.
    #[serde(rename = "check_version", skip_serializing_if = "Option::is_none")]
    pub check_version: i64,
    /// Locations for which to query the API test results.
    #[serde(rename = "probe_dc", skip_serializing_if = "Option::is_none")]
    pub probe_dc: String,
    /// Object containing results for your Synthetic API test.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: SyntheticsAPITestResultData,
    /// ID of the API test result.
    #[serde(rename = "result_id", skip_serializing_if = "Option::is_none")]
    pub result_id: String,
    /// The status of your Synthetic monitor.
* `O` for not triggered
* `1` for triggered
* `2` for no data
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: SyntheticsTestMonitorStatus,
}

