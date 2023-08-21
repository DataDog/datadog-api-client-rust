// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultData {
    /// Type of browser device used for the browser test.
    #[serde(rename = "browserType", skip_serializing_if = "Option::is_none")]
    pub browser_type: String,
    /// Browser version used for the browser test.
    #[serde(rename = "browserVersion", skip_serializing_if = "Option::is_none")]
    pub browser_version: String,
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: SyntheticsDevice,
    /// Global duration in second of the browser test.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: f64,
    /// Error returned for the browser test.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: String,
    /// The browser test failure details.
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: SyntheticsBrowserTestResultFailure,
    /// Whether or not the browser test was conducted.
    #[serde(rename = "passed", skip_serializing_if = "Option::is_none")]
    pub passed: bool,
    /// The amount of email received during the browser test.
    #[serde(rename = "receivedEmailCount", skip_serializing_if = "Option::is_none")]
    pub received_email_count: i64,
    /// Starting URL for the browser test.
    #[serde(rename = "startUrl", skip_serializing_if = "Option::is_none")]
    pub start_url: String,
    /// Array containing the different browser test steps.
    #[serde(rename = "stepDetails", skip_serializing_if = "Option::is_none")]
    pub step_details: Vec<SyntheticsStepDetail>,
    /// Whether or not a thumbnail is associated with the browser test.
    #[serde(rename = "thumbnailsBucketKey", skip_serializing_if = "Option::is_none")]
    pub thumbnails_bucket_key: bool,
    /// Time in second to wait before the browser test starts after
reaching the start URL.
    #[serde(rename = "timeToInteractive", skip_serializing_if = "Option::is_none")]
    pub time_to_interactive: f64,
}

