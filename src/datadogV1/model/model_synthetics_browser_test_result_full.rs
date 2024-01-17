// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object returned describing a browser test result.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultFull {
    /// Object describing the browser test configuration.
    #[serde(rename = "check")]
    pub check: Option<Box<crate::datadogV1::model::SyntheticsBrowserTestResultFullCheck>>,
    /// When the browser test was conducted.
    #[serde(rename = "check_time")]
    pub check_time: Option<f64>,
    /// Version of the browser test used.
    #[serde(rename = "check_version")]
    pub check_version: Option<i64>,
    /// Location from which the browser test was performed.
    #[serde(rename = "probe_dc")]
    pub probe_dc: Option<String>,
    /// Object containing results for your Synthetic browser test.
    #[serde(rename = "result")]
    pub result: Option<Box<crate::datadogV1::model::SyntheticsBrowserTestResultData>>,
    /// ID of the browser test result.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    /// The status of your Synthetic monitor.
    /// * `O` for not triggered
    /// * `1` for triggered
    /// * `2` for no data
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestMonitorStatus>,
}

impl SyntheticsBrowserTestResultFull {
    pub fn new() -> SyntheticsBrowserTestResultFull {
        SyntheticsBrowserTestResultFull {
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
impl Default for SyntheticsBrowserTestResultFull {
    fn default() -> Self {
        Self::new()
    }
}
