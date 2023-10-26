// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountCreateRequestAttributes {
    /// The API key for the Fastly account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The name of the Fastly account.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of services belonging to the parent account.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::datadogV2::model::FastlyService>>,
}

impl FastlyAccountCreateRequestAttributes {
    /// Attributes object for creating a Fastly account.
    pub fn new(api_key: String, name: String) -> FastlyAccountCreateRequestAttributes {
        FastlyAccountCreateRequestAttributes {
            api_key: api_key,
            name: name,
            services: None,
        }
    }
}
