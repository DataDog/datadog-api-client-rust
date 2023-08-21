// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAPIError {
    /// Code identifying the error
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: String,
    /// Additional error details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Vec<LogsAPIError>,
    /// Error message
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
}

