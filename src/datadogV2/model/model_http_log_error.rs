// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of errors.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPLogError {
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

impl HTTPLogError {
    pub fn new() -> HTTPLogError {
        HTTPLogError {
            detail: None,
            status: None,
            title: None,
        }
    }
}