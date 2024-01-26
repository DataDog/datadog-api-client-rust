// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Result of the last API test run.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultShortResult {
    /// Describes if the test run has passed or failed.
    #[serde(rename = "passed")]
    pub passed: Option<bool>,
    /// Object containing all metrics and their values collected for a Synthetic API test.
    /// See the [Synthetic Monitoring Metrics documentation](<https://docs.datadoghq.com/synthetics/metrics/>).
    #[serde(rename = "timings")]
    pub timings: Option<Box<crate::datadogV1::model::SyntheticsTiming>>,
}

impl SyntheticsAPITestResultShortResult {
    pub fn new() -> SyntheticsAPITestResultShortResult {
        SyntheticsAPITestResultShortResult {
            passed: None,
            timings: None,
        }
    }
}
impl Default for SyntheticsAPITestResultShortResult {
    fn default() -> Self {
        Self::new()
    }
}
