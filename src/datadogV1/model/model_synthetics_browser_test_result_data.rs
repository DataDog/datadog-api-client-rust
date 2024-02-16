// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing results for your Synthetic browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultData {
    /// Type of browser device used for the browser test.
    #[serde(rename = "browserType")]
    pub browser_type: Option<String>,
    /// Browser version used for the browser test.
    #[serde(rename = "browserVersion")]
    pub browser_version: Option<String>,
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDevice>,
    /// Global duration in second of the browser test.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Error returned for the browser test.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// The browser test failure details.
    #[serde(rename = "failure")]
    pub failure: Option<crate::datadogV1::model::SyntheticsBrowserTestResultFailure>,
    /// Whether or not the browser test was conducted.
    #[serde(rename = "passed")]
    pub passed: Option<bool>,
    /// The amount of email received during the browser test.
    #[serde(rename = "receivedEmailCount")]
    pub received_email_count: Option<i64>,
    /// Starting URL for the browser test.
    #[serde(rename = "startUrl")]
    pub start_url: Option<String>,
    /// Array containing the different browser test steps.
    #[serde(rename = "stepDetails")]
    pub step_details: Option<Vec<crate::datadogV1::model::SyntheticsStepDetail>>,
    /// Whether or not a thumbnail is associated with the browser test.
    #[serde(rename = "thumbnailsBucketKey")]
    pub thumbnails_bucket_key: Option<bool>,
    /// Time in second to wait before the browser test starts after
    /// reaching the start URL.
    #[serde(rename = "timeToInteractive")]
    pub time_to_interactive: Option<f64>,
}

impl SyntheticsBrowserTestResultData {
    pub fn new() -> SyntheticsBrowserTestResultData {
        SyntheticsBrowserTestResultData {
            browser_type: None,
            browser_version: None,
            device: None,
            duration: None,
            error: None,
            failure: None,
            passed: None,
            received_email_count: None,
            start_url: None,
            step_details: None,
            thumbnails_bucket_key: None,
            time_to_interactive: None,
        }
    }

    pub fn browser_type(&mut self, value: String) -> &mut Self {
        self.browser_type = Some(value);
        self
    }

    pub fn browser_version(&mut self, value: String) -> &mut Self {
        self.browser_version = Some(value);
        self
    }

    pub fn device(&mut self, value: crate::datadogV1::model::SyntheticsDevice) -> &mut Self {
        self.device = Some(value);
        self
    }

    pub fn duration(&mut self, value: f64) -> &mut Self {
        self.duration = Some(value);
        self
    }

    pub fn error(&mut self, value: String) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn failure(
        &mut self,
        value: crate::datadogV1::model::SyntheticsBrowserTestResultFailure,
    ) -> &mut Self {
        self.failure = Some(value);
        self
    }

    pub fn passed(&mut self, value: bool) -> &mut Self {
        self.passed = Some(value);
        self
    }

    pub fn received_email_count(&mut self, value: i64) -> &mut Self {
        self.received_email_count = Some(value);
        self
    }

    pub fn start_url(&mut self, value: String) -> &mut Self {
        self.start_url = Some(value);
        self
    }

    pub fn step_details(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsStepDetail>,
    ) -> &mut Self {
        self.step_details = Some(value);
        self
    }

    pub fn thumbnails_bucket_key(&mut self, value: bool) -> &mut Self {
        self.thumbnails_bucket_key = Some(value);
        self
    }

    pub fn time_to_interactive(&mut self, value: f64) -> &mut Self {
        self.time_to_interactive = Some(value);
        self
    }
}

impl Default for SyntheticsBrowserTestResultData {
    fn default() -> Self {
        Self::new()
    }
}
