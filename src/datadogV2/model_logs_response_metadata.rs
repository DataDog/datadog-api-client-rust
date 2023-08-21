// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsResponseMetadata {
    /// The time elapsed in milliseconds
    #[serde(rename = "elapsed", skip_serializing_if = "Option::is_none")]
    pub elapsed: i64,
    /// Paging attributes.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: LogsResponseMetadataPage,
    /// The identifier of the request
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: String,
    /// The status of the response
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: LogsAggregateResponseStatus,
    /// A list of warnings (non fatal errors) encountered, partial results might be returned if
warnings are present in the response.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Vec<LogsWarning>,
}

