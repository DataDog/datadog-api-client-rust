// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Portable PDB file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PortablePdbSourcemapAttributes {
    /// The build identifier.
    #[serde(rename = "build_id")]
    pub build_id: Option<String>,
    /// The timestamp when the symbol file was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The file name of the symbol file.
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    /// The type of source map. Always `portable_pdb` for Portable PDB files.
    #[serde(rename = "mapkind")]
    pub mapkind: crate::datadogV2::model::PortablePdbSourcemapMapKind,
    /// The size of the symbol file in bytes.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PortablePdbSourcemapAttributes {
    pub fn new(
        mapkind: crate::datadogV2::model::PortablePdbSourcemapMapKind,
    ) -> PortablePdbSourcemapAttributes {
        PortablePdbSourcemapAttributes {
            build_id: None,
            created_at: None,
            file_name: None,
            mapkind,
            size: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn build_id(mut self, value: String) -> Self {
        self.build_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn file_name(mut self, value: String) -> Self {
        self.file_name = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
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

impl<'de> Deserialize<'de> for PortablePdbSourcemapAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PortablePdbSourcemapAttributesVisitor;
        impl<'a> Visitor<'a> for PortablePdbSourcemapAttributesVisitor {
            type Value = PortablePdbSourcemapAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut build_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut file_name: Option<String> = None;
                let mut mapkind: Option<crate::datadogV2::model::PortablePdbSourcemapMapKind> =
                    None;
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
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_name" => {
                            if v.is_null() {
                                continue;
                            }
                            file_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapkind" => {
                            mapkind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mapkind) = mapkind {
                                match _mapkind {
                                    crate::datadogV2::model::PortablePdbSourcemapMapKind::UnparsedObject(_mapkind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mapkind = mapkind.ok_or_else(|| M::Error::missing_field("mapkind"))?;

                let content = PortablePdbSourcemapAttributes {
                    build_id,
                    created_at,
                    file_name,
                    mapkind,
                    size,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PortablePdbSourcemapAttributesVisitor)
    }
}
