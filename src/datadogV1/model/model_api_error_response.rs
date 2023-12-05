// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Error response object.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct APIErrorResponse {
    /// Array of errors returned by the API.
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
}

impl APIErrorResponse {
    pub fn new(errors: Vec<String>) -> APIErrorResponse {
        APIErrorResponse { errors }
    }
}