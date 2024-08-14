// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object of a Cloudflare account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudflareAccountResponseAttributes {
    /// The CloudflareAccountResponseAttributes api_key.
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    /// The email associated with the Cloudflare account.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The name of the Cloudflare account.
    #[serde(rename = "name")]
    pub name: String,
    /// An allowlist of resources to restrict pulling metrics for.
    #[serde(rename = "resources")]
    pub resources: Option<Vec<String>>,
    /// An allowlist of zones to restrict pulling metrics for.
    #[serde(rename = "zones")]
    pub zones: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudflareAccountResponseAttributes {
    pub fn new(name: String) -> CloudflareAccountResponseAttributes {
        CloudflareAccountResponseAttributes {
            api_key: None,
            email: None,
            name,
            resources: None,
            zones: None,
            _unparsed: false,
        }
    }

    pub fn api_key(mut self, value: String) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn resources(mut self, value: Vec<String>) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn zones(mut self, value: Vec<String>) -> Self {
        self.zones = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CloudflareAccountResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudflareAccountResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CloudflareAccountResponseAttributesVisitor {
            type Value = CloudflareAccountResponseAttributes;

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
                let mut resources: Option<Vec<String>> = None;
                let mut zones: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "resources" => {
                            if v.is_null() {
                                continue;
                            }
                            resources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "zones" => {
                            if v.is_null() {
                                continue;
                            }
                            zones = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CloudflareAccountResponseAttributes {
                    api_key,
                    email,
                    name,
                    resources,
                    zones,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudflareAccountResponseAttributesVisitor)
    }
}
