// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing a step for a Synthetic test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsStepDetail {
    /// Array of errors collected for a browser test.
    #[serde(rename = "browserErrors")]
    pub browser_errors: Option<Vec<crate::datadogV1::model::SyntheticsBrowserError>>,
    /// Type of assertion to apply in an API test.
    #[serde(rename = "checkType")]
    pub check_type: Option<crate::datadogV1::model::SyntheticsCheckType>,
    /// Description of the test.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Total duration in millisecond of the test.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Error returned by the test.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Navigate between different tabs for your browser test.
    #[serde(rename = "playingTab")]
    pub playing_tab: Option<crate::datadogV1::model::SyntheticsPlayingTab>,
    /// Whether or not screenshots where collected by the test.
    #[serde(rename = "screenshotBucketKey")]
    pub screenshot_bucket_key: Option<bool>,
    /// Whether or not to skip this step.
    #[serde(rename = "skipped")]
    pub skipped: Option<bool>,
    /// Whether or not snapshots where collected by the test.
    #[serde(rename = "snapshotBucketKey")]
    pub snapshot_bucket_key: Option<bool>,
    /// The step ID.
    #[serde(rename = "stepId")]
    pub step_id: Option<i64>,
    /// If this step includes a sub-test.
    /// [Subtests documentation](https://docs.datadoghq.com/synthetics/browser_tests/advanced_options/#subtests).
    #[serde(rename = "subTestStepDetails")]
    pub sub_test_step_details: Option<Vec<crate::datadogV1::model::SyntheticsStepDetail>>,
    /// Time before starting the step.
    #[serde(rename = "timeToInteractive")]
    pub time_to_interactive: Option<f64>,
    /// Step type used in your Synthetic test.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsStepType>,
    /// URL to perform the step against.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Value for the step.
    #[serde(rename = "value")]
    pub value: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Array of Core Web Vitals metrics for the step.
    #[serde(rename = "vitalsMetrics")]
    pub vitals_metrics: Option<Vec<crate::datadogV1::model::SyntheticsCoreWebVitals>>,
    /// Warning collected that didn't failed the step.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV1::model::SyntheticsStepDetailWarning>>,
}

impl SyntheticsStepDetail {
    pub fn new() -> SyntheticsStepDetail {
        SyntheticsStepDetail {
            browser_errors: None,
            check_type: None,
            description: None,
            duration: None,
            error: None,
            playing_tab: None,
            screenshot_bucket_key: None,
            skipped: None,
            snapshot_bucket_key: None,
            step_id: None,
            sub_test_step_details: None,
            time_to_interactive: None,
            type_: None,
            url: None,
            value: None,
            vitals_metrics: None,
            warnings: None,
        }
    }
}
