// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a SAML configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SAMLConfigurationUpdateAttributes {
    /// Whether identity-provider-initiated login is enabled for the organization.
    #[serde(rename = "idp_initiated")]
    pub idp_initiated: Option<bool>,
    /// Email domains for which users are automatically provisioned on first SAML login
    /// (just-in-time provisioning). A default role is required to enable just-in-time provisioning.
    #[serde(rename = "jit_domains")]
    pub jit_domains: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SAMLConfigurationUpdateAttributes {
    pub fn new() -> SAMLConfigurationUpdateAttributes {
        SAMLConfigurationUpdateAttributes {
            idp_initiated: None,
            jit_domains: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn idp_initiated(mut self, value: bool) -> Self {
        self.idp_initiated = Some(value);
        self
    }

    pub fn jit_domains(mut self, value: Vec<String>) -> Self {
        self.jit_domains = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SAMLConfigurationUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SAMLConfigurationUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SAMLConfigurationUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SAMLConfigurationUpdateAttributesVisitor {
            type Value = SAMLConfigurationUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut idp_initiated: Option<bool> = None;
                let mut jit_domains: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "idp_initiated" => {
                            if v.is_null() {
                                continue;
                            }
                            idp_initiated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jit_domains" => {
                            if v.is_null() {
                                continue;
                            }
                            jit_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SAMLConfigurationUpdateAttributes {
                    idp_initiated,
                    jit_domains,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SAMLConfigurationUpdateAttributesVisitor)
    }
}
