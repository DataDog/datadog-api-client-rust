// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettings {
    /// Whether or not the organization users can share widgets outside of Datadog.
    #[serde(rename = "private_widget_share", skip_serializing_if = "Option::is_none")]
    pub private_widget_share: bool,
    /// Set the boolean property enabled to enable or disable single sign on with SAML.
See the SAML documentation for more information about all SAML settings.
    #[serde(rename = "saml", skip_serializing_if = "Option::is_none")]
    pub saml: OrganizationSettingsSaml,
    /// The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).
    #[serde(rename = "saml_autocreate_access_role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub saml_autocreate_access_role: NullableAccessRole,
    /// Has two properties, `enabled` (boolean) and `domains`, which is a list of domains without the @ symbol.
    #[serde(rename = "saml_autocreate_users_domains", skip_serializing_if = "Option::is_none")]
    pub saml_autocreate_users_domains: OrganizationSettingsSamlAutocreateUsersDomains,
    /// Whether or not SAML can be enabled for this organization.
    #[serde(rename = "saml_can_be_enabled", skip_serializing_if = "Option::is_none")]
    pub saml_can_be_enabled: bool,
    /// Identity provider endpoint for SAML authentication.
    #[serde(rename = "saml_idp_endpoint", skip_serializing_if = "Option::is_none")]
    pub saml_idp_endpoint: String,
    /// Has one property enabled (boolean).
    #[serde(rename = "saml_idp_initiated_login", skip_serializing_if = "Option::is_none")]
    pub saml_idp_initiated_login: OrganizationSettingsSamlIdpInitiatedLogin,
    /// Whether or not a SAML identity provider metadata file was provided to the Datadog organization.
    #[serde(rename = "saml_idp_metadata_uploaded", skip_serializing_if = "Option::is_none")]
    pub saml_idp_metadata_uploaded: bool,
    /// URL for SAML logging.
    #[serde(rename = "saml_login_url", skip_serializing_if = "Option::is_none")]
    pub saml_login_url: String,
    /// Has one property enabled (boolean).
    #[serde(rename = "saml_strict_mode", skip_serializing_if = "Option::is_none")]
    pub saml_strict_mode: OrganizationSettingsSamlStrictMode,
}

