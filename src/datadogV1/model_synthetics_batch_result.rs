// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBatchResult {
    /// The device ID.
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: SyntheticsDeviceID,
    /// Total duration in millisecond of the test.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: f64,
    /// Execution rule for a Synthetic test.
    #[serde(rename = "execution_rule", skip_serializing_if = "Option::is_none")]
    pub execution_rule: SyntheticsTestExecutionRule,
    /// Name of the location.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: String,
    /// The ID of the result to get.
    #[serde(rename = "result_id", skip_serializing_if = "Option::is_none")]
    pub result_id: String,
    /// Number of times this result has been retried.
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: f64,
    /// Determines whether or not the batch has passed, failed, or is in progress.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: SyntheticsStatus,
    /// Name of the test.
    #[serde(rename = "test_name", skip_serializing_if = "Option::is_none")]
    pub test_name: String,
    /// The public ID of the Synthetic test.
    #[serde(rename = "test_public_id", skip_serializing_if = "Option::is_none")]
    pub test_public_id: String,
    /// Type of the Synthetic test, either `api` or `browser`.
    #[serde(rename = "test_type", skip_serializing_if = "Option::is_none")]
    pub test_type: SyntheticsTestDetailsType,
}

