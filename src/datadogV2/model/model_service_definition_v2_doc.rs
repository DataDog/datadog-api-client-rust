// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service documents.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Doc {
    /// Document name.
    #[serde(rename = "name")]
    pub name: String,
    /// Document provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Document URL.
    #[serde(rename = "url")]
    pub url: String,
}

impl ServiceDefinitionV2Doc {
    pub fn new(name: String, url: String) -> ServiceDefinitionV2Doc {
        ServiceDefinitionV2Doc {
            name,
            provider: None,
            url,
        }
    }

    pub fn provider(&mut self, value: String) -> &mut Self {
        self.provider = Some(value);
        self
    }
}
