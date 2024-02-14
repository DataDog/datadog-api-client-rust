// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service's external links.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1Link {
    /// Link name.
    #[serde(rename = "name")]
    pub name: String,
    /// Link provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Link type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ServiceDefinitionV2Dot1LinkType,
    /// Link URL.
    #[serde(rename = "url")]
    pub url: String,
}

impl ServiceDefinitionV2Dot1Link {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::ServiceDefinitionV2Dot1LinkType,
        url: String,
    ) -> ServiceDefinitionV2Dot1Link {
        ServiceDefinitionV2Dot1Link {
            name,
            provider: None,
            type_,
            url,
        }
    }

    pub fn provider(&mut self, value: String) -> &mut Self {
        self.provider = Some(value);
        self
    }
}
