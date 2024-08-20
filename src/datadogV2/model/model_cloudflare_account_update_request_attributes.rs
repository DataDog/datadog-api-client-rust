// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for updating a Cloudflare account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudflareAccountUpdateRequestAttributes {
    /// The API key of the Cloudflare account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The email associated with the Cloudflare account. If an API key is provided (and not a token), this field is also required.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// An allowlist of resources to restrict pulling metrics for.
    #[serde(rename = "resources")]
    pub resources: Option<Vec<String>>,
    /// An allowlist of zones to restrict pulling metrics for.
    #[serde(rename = "zones")]
    pub zones: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudflareAccountUpdateRequestAttributes {
    pub fn new(api_key: String) -> CloudflareAccountUpdateRequestAttributes {
        CloudflareAccountUpdateRequestAttributes {
            api_key,
            email: None,
            resources: None,
            zones: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CloudflareAccountUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudflareAccountUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CloudflareAccountUpdateRequestAttributesVisitor {
            type Value = CloudflareAccountUpdateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut email: Option<String> = None;
                let mut resources: Option<Vec<String>> = None;
                let mut zones: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let api_key = api_key.ok_or_else(|| M::Error::missing_field("api_key"))?;

                let content = CloudflareAccountUpdateRequestAttributes {
                    api_key,
                    email,
                    resources,
                    zones,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudflareAccountUpdateRequestAttributesVisitor)
    }
}
