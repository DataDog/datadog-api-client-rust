// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of Entity V3 Metadata Links Items object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3MetadataLinksItems {
    /// Link name
    #[serde(rename = "name")]
    pub name: String,
    /// Link provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Link type.
    #[serde(rename = "type")]
    pub type_: String,
    /// Link url
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3MetadataLinksItems {
    pub fn new(name: String, type_: String, url: String) -> EntityV3MetadataLinksItems {
        EntityV3MetadataLinksItems {
            name,
            provider: None,
            type_,
            url,
            _unparsed: false,
        }
    }

    pub fn provider(mut self, value: String) -> Self {
        self.provider = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EntityV3MetadataLinksItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3MetadataLinksItemsVisitor;
        impl<'a> Visitor<'a> for EntityV3MetadataLinksItemsVisitor {
            type Value = EntityV3MetadataLinksItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut provider: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = EntityV3MetadataLinksItems {
                    name,
                    provider,
                    type_,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3MetadataLinksItemsVisitor)
    }
}
