// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An application key with its associated metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationKey {
    /// Hash of an application key.
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    /// Name of an application key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Owner of an application key.
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationKey {
    pub fn new() -> ApplicationKey {
        ApplicationKey {
            hash: None,
            name: None,
            owner: None,
            _unparsed: false,
        }
    }

    pub fn hash(mut self, value: String) -> Self {
        self.hash = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn owner(mut self, value: String) -> Self {
        self.owner = Some(value);
        self
    }
}

impl Default for ApplicationKey {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationKeyVisitor;
        impl<'a> Visitor<'a> for ApplicationKeyVisitor {
            type Value = ApplicationKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hash: Option<String> = None;
                let mut name: Option<String> = None;
                let mut owner: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hash" => {
                            if v.is_null() {
                                continue;
                            }
                            hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner" => {
                            if v.is_null() {
                                continue;
                            }
                            owner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ApplicationKey {
                    hash,
                    name,
                    owner,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationKeyVisitor)
    }
}
