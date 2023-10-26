// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudflareAccountCreateRequestAttributes {
    /// The API key (or token) for the Cloudflare account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The email associated with the Cloudflare account. If an API key is provided (and not a token), this field is also required.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The name of the Cloudflare account.
    #[serde(rename = "name")]
    pub name: String,
}

impl CloudflareAccountCreateRequestAttributes {
    /// Attributes object for creating a Cloudflare account.
    pub fn new(api_key: String, name: String) -> CloudflareAccountCreateRequestAttributes {
        CloudflareAccountCreateRequestAttributes {
            api_key: api_key,
            email: None,
            name: name,
        }
    }
}
