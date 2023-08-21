// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Doc {
    /// Document name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Document provider.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: String,
    /// Document URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

