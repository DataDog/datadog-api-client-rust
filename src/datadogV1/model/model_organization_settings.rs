// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A JSON array of settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationSettings {
    /// Whether or not the organization users can share widgets outside of Datadog.
    #[serde(rename = "private_widget_share")]
    pub private_widget_share: Option<bool>,
    /// Set the boolean property enabled to enable or disable single sign on with SAML.
    /// See the SAML documentation for more information about all SAML settings.
    #[serde(rename = "saml")]
    pub saml: Option<crate::datadogV1::model::OrganizationSettingsSaml>,
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
        Option<crate::datadogV1::model::OrganizationSettingsSamlAutocreateUsersDomains>,
    /// Whether or not SAML can be enabled for this organization.
    #[serde(rename = "saml_can_be_enabled")]
    pub saml_can_be_enabled: Option<bool>,
    /// Identity provider endpoint for SAML authentication.
    #[serde(rename = "saml_idp_endpoint")]
    pub saml_idp_endpoint: Option<String>,
    /// Has one property enabled (boolean).
    #[serde(rename = "saml_idp_initiated_login")]
    pub saml_idp_initiated_login:
        Option<crate::datadogV1::model::OrganizationSettingsSamlIdpInitiatedLogin>,
    /// Whether or not a SAML identity provider metadata file was provided to the Datadog organization.
    #[serde(rename = "saml_idp_metadata_uploaded")]
    pub saml_idp_metadata_uploaded: Option<bool>,
    /// URL for SAML logging.
    #[serde(rename = "saml_login_url")]
    pub saml_login_url: Option<String>,
    /// Has one property enabled (boolean).
    #[serde(rename = "saml_strict_mode")]
    pub saml_strict_mode: Option<crate::datadogV1::model::OrganizationSettingsSamlStrictMode>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn private_widget_share(mut self, value: bool) -> Self {
        self.private_widget_share = Some(value);
        self
    }

    pub fn saml(mut self, value: crate::datadogV1::model::OrganizationSettingsSaml) -> Self {
        self.saml = Some(value);
        self
    }

    pub fn saml_autocreate_access_role(
        mut self,
        value: Option<crate::datadogV1::model::AccessRole>,
    ) -> Self {
        self.saml_autocreate_access_role = Some(value);
        self
    }

    pub fn saml_autocreate_users_domains(
        mut self,
        value: crate::datadogV1::model::OrganizationSettingsSamlAutocreateUsersDomains,
    ) -> Self {
        self.saml_autocreate_users_domains = Some(value);
        self
    }

    pub fn saml_can_be_enabled(mut self, value: bool) -> Self {
        self.saml_can_be_enabled = Some(value);
        self
    }

    pub fn saml_idp_endpoint(mut self, value: String) -> Self {
        self.saml_idp_endpoint = Some(value);
        self
    }

    pub fn saml_idp_initiated_login(
        mut self,
        value: crate::datadogV1::model::OrganizationSettingsSamlIdpInitiatedLogin,
    ) -> Self {
        self.saml_idp_initiated_login = Some(value);
        self
    }

    pub fn saml_idp_metadata_uploaded(mut self, value: bool) -> Self {
        self.saml_idp_metadata_uploaded = Some(value);
        self
    }

    pub fn saml_login_url(mut self, value: String) -> Self {
        self.saml_login_url = Some(value);
        self
    }

    pub fn saml_strict_mode(
        mut self,
        value: crate::datadogV1::model::OrganizationSettingsSamlStrictMode,
    ) -> Self {
        self.saml_strict_mode = Some(value);
        self
    }
}

impl Default for OrganizationSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrganizationSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationSettingsVisitor;
        impl<'a> Visitor<'a> for OrganizationSettingsVisitor {
            type Value = OrganizationSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut private_widget_share: Option<bool> = None;
                let mut saml: Option<crate::datadogV1::model::OrganizationSettingsSaml> = None;
                let mut saml_autocreate_access_role: Option<
                    Option<crate::datadogV1::model::AccessRole>,
                > = None;
                let mut saml_autocreate_users_domains: Option<
                    crate::datadogV1::model::OrganizationSettingsSamlAutocreateUsersDomains,
                > = None;
                let mut saml_can_be_enabled: Option<bool> = None;
                let mut saml_idp_endpoint: Option<String> = None;
                let mut saml_idp_initiated_login: Option<
                    crate::datadogV1::model::OrganizationSettingsSamlIdpInitiatedLogin,
                > = None;
                let mut saml_idp_metadata_uploaded: Option<bool> = None;
                let mut saml_login_url: Option<String> = None;
                let mut saml_strict_mode: Option<
                    crate::datadogV1::model::OrganizationSettingsSamlStrictMode,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "private_widget_share" => {
                            if v.is_null() {
                                continue;
                            }
                            private_widget_share =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml" => {
                            if v.is_null() {
                                continue;
                            }
                            saml = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_autocreate_access_role" => {
                            saml_autocreate_access_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _saml_autocreate_access_role) =
                                saml_autocreate_access_role
                            {
                                match _saml_autocreate_access_role {
                                    Some(crate::datadogV1::model::AccessRole::UnparsedObject(
                                        _saml_autocreate_access_role,
                                    )) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "saml_autocreate_users_domains" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_autocreate_users_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_can_be_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_can_be_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_idp_endpoint" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_idp_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_idp_initiated_login" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_idp_initiated_login =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_idp_metadata_uploaded" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_idp_metadata_uploaded =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_login_url" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_login_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_strict_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_strict_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrganizationSettings {
                    private_widget_share,
                    saml,
                    saml_autocreate_access_role,
                    saml_autocreate_users_domains,
                    saml_can_be_enabled,
                    saml_idp_endpoint,
                    saml_idp_initiated_login,
                    saml_idp_metadata_uploaded,
                    saml_login_url,
                    saml_strict_mode,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationSettingsVisitor)
    }
}
