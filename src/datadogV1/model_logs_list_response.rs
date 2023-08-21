// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListResponse {
    /// Array of logs matching the request and the `nextLogId` if sent.
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Vec<Log>,
    /// Hash identifier of the next log to return in the list.
This parameter is used for the pagination feature.
    #[serde(rename = "nextLogId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub next_log_id: Option<String>,
    /// Status of the response.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
}

