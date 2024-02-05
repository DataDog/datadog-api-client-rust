// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object describing the error with error type and error message.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryResponseErrorWithType {
    /// A message with more details about the error.
    #[serde(rename = "error_message")]
    pub error_message: String,
    /// Type of the error.
    #[serde(rename = "error_type")]
    pub error_type: String,
}

impl SLOHistoryResponseErrorWithType {
    pub fn new(error_message: String, error_type: String) -> SLOHistoryResponseErrorWithType {
        SLOHistoryResponseErrorWithType {
            error_message,
            error_type,
        }
    }
}
