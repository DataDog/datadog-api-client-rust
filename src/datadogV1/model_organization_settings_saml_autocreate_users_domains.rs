// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlAutocreateUsersDomains {
    /// List of domains where the SAML automated user creation is enabled.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Vec<String>,
    /// Whether or not the automated user creation based on SAML domain is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: bool,
}

