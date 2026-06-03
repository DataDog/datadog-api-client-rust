// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a JavaScript source map.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JSSourcemapAttributes {
    /// The absolute path to the minified JavaScript file.
    #[serde(rename = "absolute_path")]
    pub absolute_path: Option<String>,
    /// The path to the source map in blob storage.
    #[serde(rename = "blob_storage_sourcemap_path")]
    pub blob_storage_sourcemap_path: Option<String>,
    /// The build identifier.
    #[serde(rename = "build_id")]
    pub build_id: Option<String>,
    /// The timestamp when the source map was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The domain associated with the source map.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// The file name of the minified JavaScript file.
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    /// The type of source map.
    #[serde(rename = "mapkind")]
    pub mapkind: String,
    /// The service name associated with the source map.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// The size of the source map file in bytes.
    #[serde(rename = "size")]
    pub size: i64,
    /// The source map variant.
    #[serde(rename = "variant")]
    pub variant: Option<String>,
    /// The version of the service associated with the source map.
    #[serde(rename = "version")]
    pub version: Option<String>,
    /// The version code.
    #[serde(rename = "version_code")]
    pub version_code: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JSSourcemapAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        mapkind: String,
        size: i64,
    ) -> JSSourcemapAttributes {
        JSSourcemapAttributes {
            absolute_path: None,
            blob_storage_sourcemap_path: None,
            build_id: None,
            created_at,
            domain: None,
            file_name: None,
            mapkind,
            service: None,
            size,
            variant: None,
            version: None,
            version_code: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn absolute_path(mut self, value: String) -> Self {
        self.absolute_path = Some(value);
        self
    }

    pub fn blob_storage_sourcemap_path(mut self, value: String) -> Self {
        self.blob_storage_sourcemap_path = Some(value);
        self
    }

    pub fn build_id(mut self, value: String) -> Self {
        self.build_id = Some(value);
        self
    }

    pub fn domain(mut self, value: String) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn file_name(mut self, value: String) -> Self {
        self.file_name = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn variant(mut self, value: String) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }

    pub fn version_code(mut self, value: String) -> Self {
        self.version_code = Some(value);
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

impl<'de> Deserialize<'de> for JSSourcemapAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JSSourcemapAttributesVisitor;
        impl<'a> Visitor<'a> for JSSourcemapAttributesVisitor {
            type Value = JSSourcemapAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut absolute_path: Option<String> = None;
                let mut blob_storage_sourcemap_path: Option<String> = None;
                let mut build_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut domain: Option<String> = None;
                let mut file_name: Option<String> = None;
                let mut mapkind: Option<String> = None;
                let mut service: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut variant: Option<String> = None;
                let mut version: Option<String> = None;
                let mut version_code: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "absolute_path" => {
                            if v.is_null() {
                                continue;
                            }
                            absolute_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "blob_storage_sourcemap_path" => {
                            if v.is_null() {
                                continue;
                            }
                            blob_storage_sourcemap_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "build_id" => {
                            if v.is_null() {
                                continue;
                            }
                            build_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain" => {
                            if v.is_null() {
                                continue;
                            }
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_name" => {
                            if v.is_null() {
                                continue;
                            }
                            file_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapkind" => {
                            mapkind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant" => {
                            if v.is_null() {
                                continue;
                            }
                            variant = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_code" => {
                            if v.is_null() {
                                continue;
                            }
                            version_code =
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

                let content = JSSourcemapAttributes {
                    absolute_path,
                    blob_storage_sourcemap_path,
                    build_id,
                    created_at,
                    domain,
                    file_name,
                    mapkind,
                    service,
                    size,
                    variant,
                    version,
                    version_code,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JSSourcemapAttributesVisitor)
    }
}
