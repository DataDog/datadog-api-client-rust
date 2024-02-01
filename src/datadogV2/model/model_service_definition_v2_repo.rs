// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service code repositories.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Repo {
    /// Repository name.
    #[serde(rename = "name")]
    pub name: String,
    /// Repository provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Repository URL.
    #[serde(rename = "url")]
    pub url: String,
}

impl ServiceDefinitionV2Repo {
    pub fn new(name: String, url: String) -> ServiceDefinitionV2Repo {
        ServiceDefinitionV2Repo {
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
