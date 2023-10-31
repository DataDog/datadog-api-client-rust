// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A JSON array of settings.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettings {
    /// Whether or not the organization users can share widgets outside of Datadog.
    #[serde(rename = "private_widget_share")]
    pub private_widget_share: Option<bool>,
    /// Set the boolean property enabled to enable or disable single sign on with SAML.
    /// See the SAML documentation for more information about all SAML settings.
    #[serde(rename = "saml")]
    pub saml: Option<Box<crate::datadogV1::model::OrganizationSettingsSaml>>,
    /// The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).
    #[serde(
        rename = "saml_autocreate_access_role",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub saml_autocreate_access_role: Option<Option<crate::datadogV1::model::AccessRole>>,
    /// Has two properties, `enabled` (boolean) and `domains`, which is a list of domains without the @ symbol.
    #[serde(rename = "saml_autocreate_users_domains")]
    pub saml_autocreate_users_domains:
        Option<Box<crate::datadogV1::model::OrganizationSettingsSamlAutocreateUsersDomains>>,
    /// Whether or not SAML can be enabled for this organization.
    #[serde(rename = "saml_can_be_enabled")]
    pub saml_can_be_enabled: Option<bool>,
    /// Identity provider endpoint for SAML authentication.
    #[serde(rename = "saml_idp_endpoint")]
    pub saml_idp_endpoint: Option<String>,
    /// Has one property enabled (boolean).
    #[serde(rename = "saml_idp_initiated_login")]
    pub saml_idp_initiated_login: Option<Box<crate::datadogV1::model::OrganizationSettingsSamlIdpInitiatedLogin>>,
    /// Whether or not a SAML identity provider metadata file was provided to the Datadog organization.
    #[serde(rename = "saml_idp_metadata_uploaded")]
    pub saml_idp_metadata_uploaded: Option<bool>,
    /// URL for SAML logging.
    #[serde(rename = "saml_login_url")]
    pub saml_login_url: Option<String>,
    /// Has one property enabled (boolean).
    #[serde(rename = "saml_strict_mode")]
    pub saml_strict_mode: Option<Box<crate::datadogV1::model::OrganizationSettingsSamlStrictMode>>,
}

impl OrganizationSettings {
    pub fn new() -> OrganizationSettings {
        OrganizationSettings {
            private_widget_share: None,
            saml: None,
            saml_autocreate_access_role: None,
            saml_autocreate_users_domains: None,
            saml_can_be_enabled: None,
            saml_idp_endpoint: None,
            saml_idp_initiated_login: None,
            saml_idp_metadata_uploaded: None,
            saml_login_url: None,
            saml_strict_mode: None,
        }
    }
}
