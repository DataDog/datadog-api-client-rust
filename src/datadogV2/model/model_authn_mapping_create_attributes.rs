// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Key/Value pair of attributes used for create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingCreateAttributes {
    /// Key portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_key")]
    pub attribute_key: Option<String>,
    /// Value portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_value")]
    pub attribute_value: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingCreateAttributes {
    pub fn new() -> AuthNMappingCreateAttributes {
        AuthNMappingCreateAttributes {
            attribute_key: None,
            attribute_value: None,
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
}

impl Default for AuthNMappingCreateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuthNMappingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AuthNMappingCreateAttributesVisitor {
            type Value = AuthNMappingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute_key: Option<String> = None;
                let mut attribute_value: Option<String> = None;
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
                        &_ => {}
                    }
                }

                let content = AuthNMappingCreateAttributes {
                    attribute_key,
                    attribute_value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingCreateAttributesVisitor)
    }
}
