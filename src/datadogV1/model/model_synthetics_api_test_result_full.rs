// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object returned describing a API test result.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultFull {
    /// Object describing the API test configuration.
    #[serde(rename = "check")]
    pub check: Option<Box<crate::datadogV1::model::SyntheticsAPITestResultFullCheck>>,
    /// When the API test was conducted.
    #[serde(rename = "check_time")]
    pub check_time: Option<f64>,
    /// Version of the API test used.
    #[serde(rename = "check_version")]
    pub check_version: Option<i64>,
    /// Locations for which to query the API test results.
    #[serde(rename = "probe_dc")]
    pub probe_dc: Option<String>,
    /// Object containing results for your Synthetic API test.
    #[serde(rename = "result")]
    pub result: Option<Box<crate::datadogV1::model::SyntheticsAPITestResultData>>,
    /// ID of the API test result.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    /// The status of your Synthetic monitor.
    /// * `O` for not triggered
    /// * `1` for triggered
    /// * `2` for no data
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestMonitorStatus>,
}

impl SyntheticsAPITestResultFull {
    pub fn new() -> SyntheticsAPITestResultFull {
        SyntheticsAPITestResultFull {
            check: None,
            check_time: None,
            check_version: None,
            probe_dc: None,
            result: None,
            result_id: None,
            status: None,
        }
    }
}
