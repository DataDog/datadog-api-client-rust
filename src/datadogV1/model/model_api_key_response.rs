// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An API key with its associated metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    /// Datadog API key.
    #[serde(rename = "api_key")]
    pub api_key: Option<Box<crate::datadogV1::model::ApiKey>>,
}

impl ApiKeyResponse {
    pub fn new() -> ApiKeyResponse {
        ApiKeyResponse { api_key: None }
    }
}