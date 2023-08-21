// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JSONAPIErrorItem {
    /// A human-readable explanation specific to this occurrence of the error.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: String,
    /// Status code of the response.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// Short human-readable summary of the error.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}

