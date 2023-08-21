// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultShortResult {
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: SyntheticsDevice,
    /// Length in milliseconds of the browser test run.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: f64,
    /// Amount of errors collected for a single browser test run.
    #[serde(rename = "errorCount", skip_serializing_if = "Option::is_none")]
    pub error_count: i64,
    /// Amount of browser test steps completed before failing.
    #[serde(rename = "stepCountCompleted", skip_serializing_if = "Option::is_none")]
    pub step_count_completed: i64,
    /// Total amount of browser test steps.
    #[serde(rename = "stepCountTotal", skip_serializing_if = "Option::is_none")]
    pub step_count_total: i64,
}

