// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AuthN Mapping response from the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingResponse {
    /// The AuthN Mapping object returned by API.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::AuthNMapping>>,
    /// Included data in the AuthN Mapping response.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::AuthNMappingIncluded>>,
}

impl AuthNMappingResponse {
    pub fn new() -> AuthNMappingResponse {
        AuthNMappingResponse {
            data: None,
            included: None,
        }
    }
}
impl Default for AuthNMappingResponse {
    fn default() -> Self {
        Self::new()
    }
}
