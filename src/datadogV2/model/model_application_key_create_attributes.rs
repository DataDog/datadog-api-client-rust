// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create an application Key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationKeyCreateAttributes {
    /// Name of the application key.
    #[serde(rename = "name")]
    pub name: String,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option")]
    pub scopes: Option<Option<Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationKeyCreateAttributes {
    pub fn new(name: String) -> ApplicationKeyCreateAttributes {
        ApplicationKeyCreateAttributes {
            name,
            scopes: None,
            _unparsed: false,
        }
    }

    pub fn scopes(mut self, value: Option<Vec<String>>) -> Self {
        self.scopes = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ApplicationKeyCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationKeyCreateAttributesVisitor;
        impl<'a> Visitor<'a> for ApplicationKeyCreateAttributesVisitor {
            type Value = ApplicationKeyCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut scopes: Option<Option<Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes" => {
                            scopes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ApplicationKeyCreateAttributes {
                    name,
                    scopes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationKeyCreateAttributesVisitor)
    }
}
