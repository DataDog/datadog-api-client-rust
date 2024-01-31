// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Description of errors.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsAsyncError {
    /// Code properties
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Message content.
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl AWSLogsAsyncError {
    pub fn new() -> AWSLogsAsyncError {
        AWSLogsAsyncError {
            code: None,
            message: None,
        }
    }

    pub fn with_code(&mut self, value: String) -> &mut Self {
        self.code = Some(value);
        self
    }

    pub fn with_message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }
}
impl Default for AWSLogsAsyncError {
    fn default() -> Self {
        Self::new()
    }
}
