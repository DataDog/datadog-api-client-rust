// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JSONAPIErrorItem {
    /// A human-readable explanation specific to this occurrence of the error.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Status code of the response.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Short human-readable summary of the error.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl JSONAPIErrorItem {
    /// API error response body
    pub fn new() -> JSONAPIErrorItem {
        JSONAPIErrorItem {
            detail: None,
            status: None,
            title: None,
        }
    }
}
