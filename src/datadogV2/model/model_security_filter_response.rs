// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object which includes a single security filter.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterResponse {
    /// The security filter's properties.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::SecurityFilter>>,
    /// Optional metadata associated to the response.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::SecurityFilterMeta>>,
}

impl SecurityFilterResponse {
    pub fn new() -> SecurityFilterResponse {
        SecurityFilterResponse {
            data: None,
            meta: None,
        }
    }
}