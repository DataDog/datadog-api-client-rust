// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for creating a Cloudflare account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudflareAccountCreateRequestAttributes {
    /// The API key (or token) for the Cloudflare account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The email associated with the Cloudflare account. If an API key is provided (and not a token), this field is also required.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The name of the Cloudflare account.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudflareAccountCreateRequestAttributes {
    pub fn new(api_key: String, name: String) -> CloudflareAccountCreateRequestAttributes {
        CloudflareAccountCreateRequestAttributes {
            api_key,
            email: None,
            name,
            _unparsed: false,
        }
    }

    pub fn email(&mut self, value: String) -> &mut Self {
        self.email = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CloudflareAccountCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudflareAccountCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CloudflareAccountCreateRequestAttributesVisitor {
            type Value = CloudflareAccountCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut email: Option<String> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let api_key = api_key.ok_or_else(|| M::Error::missing_field("api_key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CloudflareAccountCreateRequestAttributes {
                    api_key,
                    email,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudflareAccountCreateRequestAttributesVisitor)
    }
}
