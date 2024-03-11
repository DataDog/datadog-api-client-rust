// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for updating a Confluent account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfluentAccountUpdateRequestAttributes {
    /// The API key associated with your Confluent account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The API secret associated with your Confluent account.
    #[serde(rename = "api_secret")]
    pub api_secret: String,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfluentAccountUpdateRequestAttributes {
    pub fn new(api_key: String, api_secret: String) -> ConfluentAccountUpdateRequestAttributes {
        ConfluentAccountUpdateRequestAttributes {
            api_key,
            api_secret,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ConfluentAccountUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfluentAccountUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ConfluentAccountUpdateRequestAttributesVisitor {
            type Value = ConfluentAccountUpdateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut api_secret: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_secret" => {
                            api_secret = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let api_key = api_key.ok_or_else(|| M::Error::missing_field("api_key"))?;
                let api_secret = api_secret.ok_or_else(|| M::Error::missing_field("api_secret"))?;

                let content = ConfluentAccountUpdateRequestAttributes {
                    api_key,
                    api_secret,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConfluentAccountUpdateRequestAttributesVisitor)
    }
}
