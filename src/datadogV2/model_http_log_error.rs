// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPLogError {
    /// Error message.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: String,
    /// Error code.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// Error title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}

