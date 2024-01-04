// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of errors.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPCIAppError {
    /// Error message.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Error code.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Error title.
    #[serde(rename = "title")]
    pub title: Option<String>,
}

impl HTTPCIAppError {
    pub fn new() -> HTTPCIAppError {
        HTTPCIAppError {
            detail: None,
            status: None,
            title: None,
        }
    }
}
