// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with the results of a Synthetic batch.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBatchResult {
    /// The device ID.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDeviceID>,
    /// Total duration in millisecond of the test.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Execution rule for a Synthetic test.
    #[serde(rename = "execution_rule")]
    pub execution_rule: Option<crate::datadogV1::model::SyntheticsTestExecutionRule>,
    /// Name of the location.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// The ID of the result to get.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    /// Number of times this result has been retried.
    #[serde(rename = "retries")]
    pub retries: Option<f64>,
    /// Determines whether or not the batch has passed, failed, or is in progress.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsStatus>,
    /// Name of the test.
    #[serde(rename = "test_name")]
    pub test_name: Option<String>,
    /// The public ID of the Synthetic test.
    #[serde(rename = "test_public_id")]
    pub test_public_id: Option<String>,
    /// Type of the Synthetic test, either `api` or `browser`.
    #[serde(rename = "test_type")]
    pub test_type: Option<crate::datadogV1::model::SyntheticsTestDetailsType>,
}

impl SyntheticsBatchResult {
    pub fn new() -> SyntheticsBatchResult {
        SyntheticsBatchResult {
            device: None,
            duration: None,
            execution_rule: None,
            location: None,
            result_id: None,
            retries: None,
            status: None,
            test_name: None,
            test_public_id: None,
            test_type: None,
        }
    }
}
impl Default for SyntheticsBatchResult {
    fn default() -> Self {
        Self::new()
    }
}
