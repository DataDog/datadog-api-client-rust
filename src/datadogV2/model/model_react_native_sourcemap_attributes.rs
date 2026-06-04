// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a React Native source map.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReactNativeSourcemapAttributes {
    /// The build number.
    #[serde(rename = "build_number")]
    pub build_number: Option<String>,
    /// The bundle name.
    #[serde(rename = "bundle_name")]
    pub bundle_name: Option<String>,
    /// The bundle version.
    #[serde(rename = "bundle_version")]
    pub bundle_version: Option<String>,
    /// The timestamp when the source map was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The debug identifier (UUID format).
    #[serde(rename = "debug_id")]
    pub debug_id: Option<String>,
    /// The type of source map.
    #[serde(rename = "mapkind")]
    pub mapkind: String,
    /// The platform the source map was built for (e.g., `ios`, `android`).
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    /// The service name associated with the source map.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// The size of the source map file in bytes.
    #[serde(rename = "size")]
    pub size: i64,
    /// The version of the service associated with the source map.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReactNativeSourcemapAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        mapkind: String,
        size: i64,
    ) -> ReactNativeSourcemapAttributes {
        ReactNativeSourcemapAttributes {
            build_number: None,
            bundle_name: None,
            bundle_version: None,
            created_at,
            debug_id: None,
            mapkind,
            platform: None,
            service: None,
            size,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn build_number(mut self, value: String) -> Self {
        self.build_number = Some(value);
        self
    }

    pub fn bundle_name(mut self, value: String) -> Self {
        self.bundle_name = Some(value);
        self
    }

    pub fn bundle_version(mut self, value: String) -> Self {
        self.bundle_version = Some(value);
        self
    }

    pub fn debug_id(mut self, value: String) -> Self {
        self.debug_id = Some(value);
        self
    }

    pub fn platform(mut self, value: String) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for ReactNativeSourcemapAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReactNativeSourcemapAttributesVisitor;
        impl<'a> Visitor<'a> for ReactNativeSourcemapAttributesVisitor {
            type Value = ReactNativeSourcemapAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut build_number: Option<String> = None;
                let mut bundle_name: Option<String> = None;
                let mut bundle_version: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut debug_id: Option<String> = None;
                let mut mapkind: Option<String> = None;
                let mut platform: Option<String> = None;
                let mut service: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "build_number" => {
                            if v.is_null() {
                                continue;
                            }
                            build_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bundle_name" => {
                            if v.is_null() {
                                continue;
                            }
                            bundle_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bundle_version" => {
                            if v.is_null() {
                                continue;
                            }
                            bundle_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "debug_id" => {
                            if v.is_null() {
                                continue;
                            }
                            debug_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapkind" => {
                            mapkind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "platform" => {
                            if v.is_null() {
                                continue;
                            }
                            platform = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ReactNativeSourcemapAttributes {
                    build_number,
                    bundle_name,
                    bundle_version,
                    created_at,
                    debug_id,
                    mapkind,
                    platform,
                    service,
                    size,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReactNativeSourcemapAttributesVisitor)
    }
}
