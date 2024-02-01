// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for retrieving an API key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIKeyResponse {
    /// Datadog API key.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::FullAPIKey>,
    /// Array of objects related to the API key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::APIKeyResponseIncludedItem>>,
}

impl APIKeyResponse {
    pub fn new() -> APIKeyResponse {
        APIKeyResponse {
            data: None,
            included: None,
        }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::FullAPIKey) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::APIKeyResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}

impl Default for APIKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}
