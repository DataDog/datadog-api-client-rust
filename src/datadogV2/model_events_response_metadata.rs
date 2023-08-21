// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsResponseMetadata {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed", skip_serializing_if = "Option::is_none")]
    pub elapsed: i64,
    /// Pagination attributes.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: EventsResponseMetadataPage,
    /// The identifier of the request.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: String,
    /// The request status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// A list of warnings (non-fatal errors) encountered. Partial results might be returned if
warnings are present in the response.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Vec<EventsWarning>,
}

