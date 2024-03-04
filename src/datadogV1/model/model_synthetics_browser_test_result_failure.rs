// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The browser test failure details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultFailure {
    /// Error code that can be returned by a Synthetic test.
    #[serde(rename = "code")]
    pub code: Option<crate::datadogV1::model::SyntheticsBrowserTestFailureCode>,
    /// The browser test error message.
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl SyntheticsBrowserTestResultFailure {
    pub fn new() -> SyntheticsBrowserTestResultFailure {
        SyntheticsBrowserTestResultFailure {
            code: None,
            message: None,
        }
    }

    pub fn code(
        mut self,
        value: crate::datadogV1::model::SyntheticsBrowserTestFailureCode,
    ) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}

impl Default for SyntheticsBrowserTestResultFailure {
    fn default() -> Self {
        Self::new()
    }
}
