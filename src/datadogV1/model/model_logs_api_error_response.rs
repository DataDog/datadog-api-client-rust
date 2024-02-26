// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response returned by the Logs API when errors occur.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAPIErrorResponse {
    /// Error returned by the Logs API
    #[serde(rename = "error")]
    pub error: Option<crate::datadogV1::model::LogsAPIError>,
}

impl LogsAPIErrorResponse {
    pub fn new() -> LogsAPIErrorResponse {
        LogsAPIErrorResponse { error: None }
    }

    pub fn error(&mut self, value: crate::datadogV1::model::LogsAPIError) -> &mut Self {
        self.error = Some(value);
        self
    }
}

impl Default for LogsAPIErrorResponse {
    fn default() -> Self {
        Self::new()
    }
}
