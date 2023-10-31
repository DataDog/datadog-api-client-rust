// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Invalid query performed.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPLogError {
    /// Error code.
    #[serde(rename = "code")]
    pub code: i32,
    /// Error message.
    #[serde(rename = "message")]
    pub message: String,
}

impl HTTPLogError {
    pub fn new(code: i32, message: String) -> HTTPLogError {
        HTTPLogError { code, message }
    }
}
