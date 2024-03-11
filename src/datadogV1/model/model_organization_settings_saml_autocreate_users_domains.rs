// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Has two properties, `enabled` (boolean) and `domains`, which is a list of domains without the @ symbol.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationSettingsSamlAutocreateUsersDomains {
    /// List of domains where the SAML automated user creation is enabled.
    #[serde(rename = "domains")]
    pub domains: Option<Vec<String>>,
    /// Whether or not the automated user creation based on SAML domain is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrganizationSettingsSamlAutocreateUsersDomains {
    pub fn new() -> OrganizationSettingsSamlAutocreateUsersDomains {
        OrganizationSettingsSamlAutocreateUsersDomains {
            domains: None,
            enabled: None,
            _unparsed: false,
        }
    }

    pub fn domains(&mut self, value: Vec<String>) -> &mut Self {
        self.domains = Some(value);
        self
    }

    pub fn enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for OrganizationSettingsSamlAutocreateUsersDomains {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrganizationSettingsSamlAutocreateUsersDomains {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationSettingsSamlAutocreateUsersDomainsVisitor;
        impl<'a> Visitor<'a> for OrganizationSettingsSamlAutocreateUsersDomainsVisitor {
            type Value = OrganizationSettingsSamlAutocreateUsersDomains;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domains: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domains" => {
                            if v.is_null() {
                                continue;
                            }
                            domains = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrganizationSettingsSamlAutocreateUsersDomains {
                    domains,
                    enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationSettingsSamlAutocreateUsersDomainsVisitor)
    }
}
