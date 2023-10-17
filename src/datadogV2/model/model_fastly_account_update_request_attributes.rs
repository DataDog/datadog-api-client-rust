// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountUpdateRequestAttributes {
    /// The API key of the Fastly account.
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

impl FastlyAccountUpdateRequestAttributes {
    /// Attributes object for updating a Fastly account.
    pub fn new() -> FastlyAccountUpdateRequestAttributes {
        FastlyAccountUpdateRequestAttributes { api_key: None }
    }
}
