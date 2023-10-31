// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object with all logs matching the request and pagination information.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsListResponse {
    /// Array of logs matching the request and the `nextLogId` if sent.
    #[serde(rename = "logs")]
    pub logs: Option<Vec<crate::datadogV1::model::Log>>,
    /// Hash identifier of the next log to return in the list.
    /// This parameter is used for the pagination feature.
    #[serde(rename = "nextLogId", default, with = "::serde_with::rust::double_option")]
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
}
