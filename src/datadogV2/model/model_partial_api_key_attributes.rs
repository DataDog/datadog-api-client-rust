// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a partial API key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PartialAPIKeyAttributes {
    /// The category of the API key.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Creation date of the API key.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The last four characters of the API key.
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    /// Date the API key was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Name of the API key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The remote config read enabled status.
    #[serde(rename = "remote_config_read_enabled")]
    pub remote_config_read_enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PartialAPIKeyAttributes {
    pub fn new() -> PartialAPIKeyAttributes {
        PartialAPIKeyAttributes {
            category: None,
            created_at: None,
            last4: None,
            modified_at: None,
            name: None,
            remote_config_read_enabled: None,
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn last4(mut self, value: String) -> Self {
        self.last4 = Some(value);
        self
    }

    pub fn modified_at(mut self, value: String) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn remote_config_read_enabled(mut self, value: bool) -> Self {
        self.remote_config_read_enabled = Some(value);
        self
    }
}

impl Default for PartialAPIKeyAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PartialAPIKeyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PartialAPIKeyAttributesVisitor;
        impl<'a> Visitor<'a> for PartialAPIKeyAttributesVisitor {
            type Value = PartialAPIKeyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut last4: Option<String> = None;
                let mut modified_at: Option<String> = None;
                let mut name: Option<String> = None;
                let mut remote_config_read_enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last4" => {
                            if v.is_null() {
                                continue;
                            }
                            last4 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_config_read_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_config_read_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = PartialAPIKeyAttributes {
                    category,
                    created_at,
                    last4,
                    modified_at,
                    name,
                    remote_config_read_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PartialAPIKeyAttributesVisitor)
    }
}
