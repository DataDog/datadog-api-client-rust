// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// API error response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JSONAPIErrorResponse {
    /// A list of errors.
    #[serde(rename = "errors")]
    pub errors: Vec<crate::datadogV2::model::JSONAPIErrorItem>,
}

impl JSONAPIErrorResponse {
    pub fn new(errors: Vec<crate::datadogV2::model::JSONAPIErrorItem>) -> JSONAPIErrorResponse {
        JSONAPIErrorResponse { errors }
    }
}
