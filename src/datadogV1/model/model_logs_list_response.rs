// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object with all logs matching the request and pagination information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListResponse {
    /// Array of logs matching the request and the `nextLogId` if sent.
    #[serde(rename = "logs")]
    pub logs: Option<Vec<crate::datadogV1::model::Log>>,
    /// Hash identifier of the next log to return in the list.
    /// This parameter is used for the pagination feature.
    #[serde(
        rename = "nextLogId",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_log_id: Option<Option<String>>,
    /// Status of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl LogsListResponse {
    pub fn new() -> LogsListResponse {
        LogsListResponse {
            logs: None,
            next_log_id: None,
            status: None,
        }
    }

    pub fn logs(&mut self, value: Vec<crate::datadogV1::model::Log>) -> &mut Self {
        self.logs = Some(value);
        self
    }

    pub fn next_log_id(&mut self, value: Option<String>) -> &mut Self {
        self.next_log_id = Some(value);
        self
    }

    pub fn status(&mut self, value: String) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for LogsListResponse {
    fn default() -> Self {
        Self::new()
    }
}
