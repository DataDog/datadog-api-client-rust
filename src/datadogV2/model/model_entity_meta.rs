// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Entity metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityMeta {
    /// The creation time.
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    /// The ingestion source.
    #[serde(rename = "ingestionSource")]
    pub ingestion_source: Option<String>,
    /// The modification time.
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    /// The origin.
    #[serde(rename = "origin")]
    pub origin: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityMeta {
    pub fn new() -> EntityMeta {
        EntityMeta {
            created_at: None,
            ingestion_source: None,
            modified_at: None,
            origin: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn ingestion_source(mut self, value: String) -> Self {
        self.ingestion_source = Some(value);
        self
    }

    pub fn modified_at(mut self, value: String) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn origin(mut self, value: String) -> Self {
        self.origin = Some(value);
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

impl Default for EntityMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityMetaVisitor;
        impl<'a> Visitor<'a> for EntityMetaVisitor {
            type Value = EntityMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut ingestion_source: Option<String> = None;
                let mut modified_at: Option<String> = None;
                let mut origin: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingestionSource" => {
                            if v.is_null() {
                                continue;
                            }
                            ingestion_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modifiedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin" => {
                            if v.is_null() {
                                continue;
                            }
                            origin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EntityMeta {
                    created_at,
                    ingestion_source,
                    modified_at,
                    origin,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityMetaVisitor)
    }
}
