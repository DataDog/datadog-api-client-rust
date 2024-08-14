// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to update an API Key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct APIKeyUpdateAttributes {
    /// The APIKeyUpdateAttributes category.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Name of the API key.
    #[serde(rename = "name")]
    pub name: String,
    /// The APIKeyUpdateAttributes remote_config_read_enabled.
    #[serde(rename = "remote_config_read_enabled")]
    pub remote_config_read_enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl APIKeyUpdateAttributes {
    pub fn new(name: String) -> APIKeyUpdateAttributes {
        APIKeyUpdateAttributes {
            category: None,
            name,
            remote_config_read_enabled: None,
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn remote_config_read_enabled(mut self, value: bool) -> Self {
        self.remote_config_read_enabled = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for APIKeyUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct APIKeyUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for APIKeyUpdateAttributesVisitor {
            type Value = APIKeyUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
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
                        "name" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = APIKeyUpdateAttributes {
                    category,
                    name,
                    remote_config_read_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(APIKeyUpdateAttributesVisitor)
    }
}
