// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for a list of API keys.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIKeysResponse {
    /// Array of API keys.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::PartialAPIKey>>,
    /// Array of objects related to the API key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::APIKeyResponseIncludedItem>>,
    /// Additional information related to api keys response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::APIKeysResponseMeta>,
}

impl APIKeysResponse {
    pub fn new() -> APIKeysResponse {
        APIKeysResponse {
            data: None,
            included: None,
            meta: None,
        }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::PartialAPIKey>) -> &mut Self {
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

    pub fn meta(&mut self, value: crate::datadogV2::model::APIKeysResponseMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for APIKeysResponse {
    fn default() -> Self {
        Self::new()
    }
}
