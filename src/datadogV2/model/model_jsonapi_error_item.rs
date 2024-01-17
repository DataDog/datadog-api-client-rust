// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// API error response body
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JSONAPIErrorItem {
    /// A human-readable explanation specific to this occurrence of the error.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Status code of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Short human-readable summary of the error.
    #[serde(rename = "title")]
    pub title: Option<String>,
}

impl JSONAPIErrorItem {
    pub fn new() -> JSONAPIErrorItem {
        JSONAPIErrorItem {
            detail: None,
            status: None,
            title: None,
        }
    }
}
impl Default for JSONAPIErrorItem {
    fn default() -> Self {
        Self::new()
    }
}
