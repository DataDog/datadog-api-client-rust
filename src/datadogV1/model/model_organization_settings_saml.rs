// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Set the boolean property enabled to enable or disable single sign on with SAML.
/// See the SAML documentation for more information about all SAML settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationSettingsSaml {
    /// Whether or not SAML is enabled for this organization.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrganizationSettingsSaml {
    pub fn new() -> OrganizationSettingsSaml {
        OrganizationSettingsSaml {
            enabled: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for OrganizationSettingsSaml {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrganizationSettingsSaml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationSettingsSamlVisitor;
        impl<'a> Visitor<'a> for OrganizationSettingsSamlVisitor {
            type Value = OrganizationSettingsSaml;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrganizationSettingsSaml { enabled, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationSettingsSamlVisitor)
    }
}
