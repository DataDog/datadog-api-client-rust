// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultFailure {
    /// Error code that can be returned by a Synthetic test.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: SyntheticsBrowserTestFailureCode,
    /// The browser test error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
}

