// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsStepDetail {
    /// Array of errors collected for a browser test.
    #[serde(rename = "browserErrors", skip_serializing_if = "Option::is_none")]
    pub browser_errors: Vec<SyntheticsBrowserError>,
    /// Type of assertion to apply in an API test.
    #[serde(rename = "checkType", skip_serializing_if = "Option::is_none")]
    pub check_type: SyntheticsCheckType,
    /// Description of the test.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Total duration in millisecond of the test.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: f64,
    /// Error returned by the test.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: String,
    /// Navigate between different tabs for your browser test.
    #[serde(rename = "playingTab", skip_serializing_if = "Option::is_none")]
    pub playing_tab: SyntheticsPlayingTab,
    /// Whether or not screenshots where collected by the test.
    #[serde(rename = "screenshotBucketKey", skip_serializing_if = "Option::is_none")]
    pub screenshot_bucket_key: bool,
    /// Whether or not to skip this step.
    #[serde(rename = "skipped", skip_serializing_if = "Option::is_none")]
    pub skipped: bool,
    /// Whether or not snapshots where collected by the test.
    #[serde(rename = "snapshotBucketKey", skip_serializing_if = "Option::is_none")]
    pub snapshot_bucket_key: bool,
    /// The step ID.
    #[serde(rename = "stepId", skip_serializing_if = "Option::is_none")]
    pub step_id: i64,
    /// If this step includes a sub-test.
[Subtests documentation](https://docs.datadoghq.com/synthetics/browser_tests/advanced_options/#subtests).
    #[serde(rename = "subTestStepDetails", skip_serializing_if = "Option::is_none")]
    pub sub_test_step_details: Vec<SyntheticsStepDetail>,
    /// Time before starting the step.
    #[serde(rename = "timeToInteractive", skip_serializing_if = "Option::is_none")]
    pub time_to_interactive: f64,
    /// Step type used in your Synthetic test.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsStepType,
    /// URL to perform the step against.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
    /// Value for the step.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: interface{},
    /// Array of Core Web Vitals metrics for the step.
    #[serde(rename = "vitalsMetrics", skip_serializing_if = "Option::is_none")]
    pub vitals_metrics: Vec<SyntheticsCoreWebVitals>,
    /// Warning collected that didn't failed the step.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Vec<SyntheticsStepDetailWarning>,
}

