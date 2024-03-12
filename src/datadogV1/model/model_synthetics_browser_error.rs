// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Error response object for a browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserError {
    /// Description of the error.
    #[serde(rename = "description")]
    pub description: String,
    /// Name of the error.
    #[serde(rename = "name")]
    pub name: String,
    /// Status Code of the error.
    #[serde(rename = "status")]
    pub status: Option<i64>,
    /// Error type returned by a browser test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBrowserErrorType,
}

impl SyntheticsBrowserError {
    pub fn new(
        description: String,
        name: String,
        type_: crate::datadogV1::model::SyntheticsBrowserErrorType,
    ) -> SyntheticsBrowserError {
        SyntheticsBrowserError {
            description,
            name,
            status: None,
            type_,
        }
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }
}
