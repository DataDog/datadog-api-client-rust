// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1Link {
    /// Link name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Link provider.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: String,
    /// Link type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ServiceDefinitionV2Dot1LinkType,
    /// Link URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

