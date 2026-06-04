// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an IL2CPP mapping file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IL2CPPSourcemapAttributes {
    /// The build identifier (UUID format).
    #[serde(rename = "build_id")]
    pub build_id: Option<String>,
    /// The timestamp when the mapping file was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The type of source map.
    #[serde(rename = "mapkind")]
    pub mapkind: String,
    /// The size of the mapping file in bytes.
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IL2CPPSourcemapAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        mapkind: String,
        size: i64,
    ) -> IL2CPPSourcemapAttributes {
        IL2CPPSourcemapAttributes {
            build_id: None,
            created_at,
            mapkind,
            size,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn build_id(mut self, value: String) -> Self {
        self.build_id = Some(value);
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

impl<'de> Deserialize<'de> for IL2CPPSourcemapAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IL2CPPSourcemapAttributesVisitor;
        impl<'a> Visitor<'a> for IL2CPPSourcemapAttributesVisitor {
            type Value = IL2CPPSourcemapAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut build_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut mapkind: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "build_id" => {
                            if v.is_null() {
                                continue;
                            }
                            build_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapkind" => {
                            mapkind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let mapkind = mapkind.ok_or_else(|| M::Error::missing_field("mapkind"))?;
                let size = size.ok_or_else(|| M::Error::missing_field("size"))?;

                let content = IL2CPPSourcemapAttributes {
                    build_id,
                    created_at,
                    mapkind,
                    size,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IL2CPPSourcemapAttributesVisitor)
    }
}
