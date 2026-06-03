// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an ELF symbol file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ELFSourcemapAttributes {
    /// The target CPU architecture.
    #[serde(rename = "arch")]
    pub arch: Option<String>,
    /// The timestamp when the symbol file was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The SHA256 hash of the ELF file.
    #[serde(rename = "file_hash")]
    pub file_hash: Option<String>,
    /// The ELF file name.
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    /// The GNU build ID (UUID format).
    #[serde(rename = "gnu_build_id")]
    pub gnu_build_id: Option<String>,
    /// The Go build ID (UUID format).
    #[serde(rename = "go_build_id")]
    pub go_build_id: Option<String>,
    /// The type of source map.
    #[serde(rename = "mapkind")]
    pub mapkind: String,
    /// The origin of the ELF file.
    #[serde(rename = "origin")]
    pub origin: Option<String>,
    /// The version of the origin package.
    #[serde(rename = "origin_version")]
    pub origin_version: Option<String>,
    /// The size of the ELF file in bytes.
    #[serde(rename = "size")]
    pub size: i64,
    /// The source of the debug symbols.
    #[serde(rename = "symbol_source")]
    pub symbol_source: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ELFSourcemapAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        mapkind: String,
        size: i64,
    ) -> ELFSourcemapAttributes {
        ELFSourcemapAttributes {
            arch: None,
            created_at,
            file_hash: None,
            file_name: None,
            gnu_build_id: None,
            go_build_id: None,
            mapkind,
            origin: None,
            origin_version: None,
            size,
            symbol_source: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn arch(mut self, value: String) -> Self {
        self.arch = Some(value);
        self
    }

    pub fn file_hash(mut self, value: String) -> Self {
        self.file_hash = Some(value);
        self
    }

    pub fn file_name(mut self, value: String) -> Self {
        self.file_name = Some(value);
        self
    }

    pub fn gnu_build_id(mut self, value: String) -> Self {
        self.gnu_build_id = Some(value);
        self
    }

    pub fn go_build_id(mut self, value: String) -> Self {
        self.go_build_id = Some(value);
        self
    }

    pub fn origin(mut self, value: String) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn origin_version(mut self, value: String) -> Self {
        self.origin_version = Some(value);
        self
    }

    pub fn symbol_source(mut self, value: String) -> Self {
        self.symbol_source = Some(value);
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

impl<'de> Deserialize<'de> for ELFSourcemapAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ELFSourcemapAttributesVisitor;
        impl<'a> Visitor<'a> for ELFSourcemapAttributesVisitor {
            type Value = ELFSourcemapAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arch: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut file_hash: Option<String> = None;
                let mut file_name: Option<String> = None;
                let mut gnu_build_id: Option<String> = None;
                let mut go_build_id: Option<String> = None;
                let mut mapkind: Option<String> = None;
                let mut origin: Option<String> = None;
                let mut origin_version: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut symbol_source: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arch" => {
                            if v.is_null() {
                                continue;
                            }
                            arch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_hash" => {
                            if v.is_null() {
                                continue;
                            }
                            file_hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_name" => {
                            if v.is_null() {
                                continue;
                            }
                            file_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gnu_build_id" => {
                            if v.is_null() {
                                continue;
                            }
                            gnu_build_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "go_build_id" => {
                            if v.is_null() {
                                continue;
                            }
                            go_build_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapkind" => {
                            mapkind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin" => {
                            if v.is_null() {
                                continue;
                            }
                            origin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin_version" => {
                            if v.is_null() {
                                continue;
                            }
                            origin_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "symbol_source" => {
                            if v.is_null() {
                                continue;
                            }
                            symbol_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ELFSourcemapAttributes {
                    arch,
                    created_at,
                    file_hash,
                    file_name,
                    gnu_build_id,
                    go_build_id,
                    mapkind,
                    origin,
                    origin_version,
                    size,
                    symbol_source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ELFSourcemapAttributesVisitor)
    }
}
