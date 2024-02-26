// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with the result of the last browser test run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultShortResult {
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDevice>,
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

    pub fn device(&mut self, value: crate::datadogV1::model::SyntheticsDevice) -> &mut Self {
        self.device = Some(value);
        self
    }

    pub fn duration(&mut self, value: f64) -> &mut Self {
        self.duration = Some(value);
        self
    }

    pub fn error_count(&mut self, value: i64) -> &mut Self {
        self.error_count = Some(value);
        self
    }

    pub fn step_count_completed(&mut self, value: i64) -> &mut Self {
        self.step_count_completed = Some(value);
        self
    }

    pub fn step_count_total(&mut self, value: i64) -> &mut Self {
        self.step_count_total = Some(value);
        self
    }
}

impl Default for SyntheticsBrowserTestResultShortResult {
    fn default() -> Self {
        Self::new()
    }
}
