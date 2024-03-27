// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Project attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectAttributes {
    /// The project's key
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Project's name
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectAttributes {
    pub fn new() -> ProjectAttributes {
        ProjectAttributes {
            key: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for ProjectAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectAttributesVisitor;
        impl<'a> Visitor<'a> for ProjectAttributesVisitor {
            type Value = ProjectAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        &_ => {}
                    }
                }

                let content = ProjectAttributes {
                    key,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectAttributesVisitor)
    }
}
