// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlStrictMode {
    /// Whether or not the SAML strict mode is enabled. If true, all users must log in with SAML.
Learn more on the [SAML Strict documentation](https://docs.datadoghq.com/account_management/saml/#saml-strict).
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: bool,
}

