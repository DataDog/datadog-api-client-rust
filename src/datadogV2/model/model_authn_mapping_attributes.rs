// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of AuthN Mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingAttributes {
    /// Key portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_key")]
    pub attribute_key: Option<String>,
    /// Value portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_value")]
    pub attribute_value: Option<String>,
    /// Creation time of the AuthN Mapping.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Time of last AuthN Mapping modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the SAML assertion attribute.
    #[serde(rename = "saml_assertion_attribute_id")]
    pub saml_assertion_attribute_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingAttributes {
    pub fn new() -> AuthNMappingAttributes {
        AuthNMappingAttributes {
            attribute_key: None,
            attribute_value: None,
            created_at: None,
            modified_at: None,
            saml_assertion_attribute_id: None,
            _unparsed: false,
        }
    }

    pub fn attribute_key(mut self, value: String) -> Self {
        self.attribute_key = Some(value);
        self
    }

    pub fn attribute_value(mut self, value: String) -> Self {
        self.attribute_value = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn saml_assertion_attribute_id(mut self, value: String) -> Self {
        self.saml_assertion_attribute_id = Some(value);
        self
    }
}

impl Default for AuthNMappingAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuthNMappingAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingAttributesVisitor;
        impl<'a> Visitor<'a> for AuthNMappingAttributesVisitor {
            type Value = AuthNMappingAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute_key: Option<String> = None;
                let mut attribute_value: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut saml_assertion_attribute_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute_key" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attribute_value" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_assertion_attribute_id" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_assertion_attribute_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AuthNMappingAttributes {
                    attribute_key,
                    attribute_value,
                    created_at,
                    modified_at,
                    saml_assertion_attribute_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingAttributesVisitor)
    }
}
