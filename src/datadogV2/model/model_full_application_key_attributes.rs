// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a full application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FullApplicationKeyAttributes {
    /// Creation date of the application key.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The application key.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The last four characters of the application key.
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    /// Name of the application key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option")]
    pub scopes: Option<Option<Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FullApplicationKeyAttributes {
    pub fn new() -> FullApplicationKeyAttributes {
        FullApplicationKeyAttributes {
            created_at: None,
            key: None,
            last4: None,
            name: None,
            scopes: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn last4(mut self, value: String) -> Self {
        self.last4 = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn scopes(mut self, value: Option<Vec<String>>) -> Self {
        self.scopes = Some(value);
        self
    }
}

impl Default for FullApplicationKeyAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FullApplicationKeyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FullApplicationKeyAttributesVisitor;
        impl<'a> Visitor<'a> for FullApplicationKeyAttributesVisitor {
            type Value = FullApplicationKeyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut key: Option<String> = None;
                let mut last4: Option<String> = None;
                let mut name: Option<String> = None;
                let mut scopes: Option<Option<Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last4" => {
                            if v.is_null() {
                                continue;
                            }
                            last4 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes" => {
                            scopes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = FullApplicationKeyAttributes {
                    created_at,
                    key,
                    last4,
                    name,
                    scopes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FullApplicationKeyAttributesVisitor)
    }
}
