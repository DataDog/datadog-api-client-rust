// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with the result of the last browser test run.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultShortResult {
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: Option<Box<crate::datadogV1::model::SyntheticsDevice>>,
    /// Length in milliseconds of the browser test run.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Amount of errors collected for a single browser test run.
    #[serde(rename = "errorCount")]
    pub error_count: Option<i64>,
    /// Amount of browser test steps completed before failing.
    #[serde(rename = "stepCountCompleted")]
    pub step_count_completed: Option<i64>,
    /// Total amount of browser test steps.
    #[serde(rename = "stepCountTotal")]
    pub step_count_total: Option<i64>,
}

impl SyntheticsBrowserTestResultShortResult {
    pub fn new() -> SyntheticsBrowserTestResultShortResult {
        SyntheticsBrowserTestResultShortResult {
            device: None,
            duration: None,
            error_count: None,
            step_count_completed: None,
            step_count_total: None,
        }
    }
}
impl Default for SyntheticsBrowserTestResultShortResult {
    fn default() -> Self {
        Self::new()
    }
}
