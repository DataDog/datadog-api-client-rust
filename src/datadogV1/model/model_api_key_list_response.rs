// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of API and application keys available for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyListResponse {
    /// Array of API keys.
    #[serde(rename = "api_keys")]
    pub api_keys: Option<Vec<crate::datadogV1::model::ApiKey>>,
}

impl ApiKeyListResponse {
    pub fn new() -> ApiKeyListResponse {
        ApiKeyListResponse { api_keys: None }
    }
}
impl Default for ApiKeyListResponse {
    fn default() -> Self {
        Self::new()
    }
}
