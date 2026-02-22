// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request to update an existing client token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClientTokenUpdateRequest {
    /// Hash value of the client token to update.
    #[serde(rename = "hash")]
    pub hash: String,
    /// New name for the client token.
    #[serde(rename = "name")]
    pub name: String,
    /// New list of allowed origin URLs. If provided, this will replace the existing list.
    #[serde(rename = "origin_urls")]
    pub origin_urls: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ClientTokenUpdateRequest {
    pub fn new(hash: String, name: String) -> ClientTokenUpdateRequest {
        ClientTokenUpdateRequest {
            hash,
            name,
            origin_urls: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn origin_urls(mut self, value: Vec<String>) -> Self {
        self.origin_urls = Some(value);
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

impl<'de> Deserialize<'de> for ClientTokenUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClientTokenUpdateRequestVisitor;
        impl<'a> Visitor<'a> for ClientTokenUpdateRequestVisitor {
            type Value = ClientTokenUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hash: Option<String> = None;
                let mut name: Option<String> = None;
                let mut origin_urls: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hash" => {
                            hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin_urls" => {
                            if v.is_null() {
                                continue;
                            }
                            origin_urls =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let hash = hash.ok_or_else(|| M::Error::missing_field("hash"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ClientTokenUpdateRequest {
                    hash,
                    name,
                    origin_urls,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ClientTokenUpdateRequestVisitor)
    }
}
