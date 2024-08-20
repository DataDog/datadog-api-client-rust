// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Datadog API key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApiKey {
    /// Date of creation of the API key.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Datadog user handle that created the API key.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// API key.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Name of your API key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApiKey {
    pub fn new() -> ApiKey {
        ApiKey {
            created: None,
            created_by: None,
            key: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created(mut self, value: String) -> Self {
        self.created = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for ApiKey {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApiKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApiKeyVisitor;
        impl<'a> Visitor<'a> for ApiKeyVisitor {
            type Value = ApiKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<String> = None;
                let mut created_by: Option<String> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApiKey {
                    created,
                    created_by,
                    key,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApiKeyVisitor)
    }
}
