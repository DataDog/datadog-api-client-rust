// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with the results of a single Synthetic API test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultShort {
    /// Last time the API test was performed.
    #[serde(rename = "check_time")]
    pub check_time: Option<f64>,
    /// Location from which the API test was performed.
    #[serde(rename = "probe_dc")]
    pub probe_dc: Option<String>,
    /// Result of the last API test run.
    #[serde(rename = "result")]
    pub result: Option<Box<crate::datadogV1::model::SyntheticsAPITestResultShortResult>>,
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

impl SyntheticsAPITestResultShort {
    pub fn new() -> SyntheticsAPITestResultShort {
        SyntheticsAPITestResultShort {
            check_time: None,
            probe_dc: None,
            result: None,
            result_id: None,
            status: None,
        }
    }
}