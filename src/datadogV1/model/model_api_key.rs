// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Datadog API key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    /// Date of creation of the API key.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Datadog user handle that created the API key.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// API key.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Name of your API key.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl ApiKey {
    pub fn new() -> ApiKey {
        ApiKey {
            created: None,
            created_by: None,
            key: None,
            name: None,
        }
    }

    pub fn with_created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn with_created_by(&mut self, value: String) -> &mut Self {
        self.created_by = Some(value);
        self
    }

    pub fn with_key(&mut self, value: String) -> &mut Self {
        self.key = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
impl Default for ApiKey {
    fn default() -> Self {
        Self::new()
    }
}
