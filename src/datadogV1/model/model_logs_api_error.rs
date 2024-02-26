// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Error returned by the Logs API
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAPIError {
    /// Code identifying the error
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Additional error details
    #[serde(rename = "details")]
    pub details: Option<Vec<crate::datadogV1::model::LogsAPIError>>,
    /// Error message
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl LogsAPIError {
    pub fn new() -> LogsAPIError {
        LogsAPIError {
            code: None,
            details: None,
            message: None,
        }
    }

    pub fn code(&mut self, value: String) -> &mut Self {
        self.code = Some(value);
        self
    }

    pub fn details(&mut self, value: Vec<crate::datadogV1::model::LogsAPIError>) -> &mut Self {
        self.details = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }
}

impl Default for LogsAPIError {
    fn default() -> Self {
        Self::new()
    }
}
