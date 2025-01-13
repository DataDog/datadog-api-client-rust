// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object for an organization's custom frameworks.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomFrameworkMetadata {
    /// Framework Creation Date
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// Framework Creator
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Framework Description
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Framework Handle
    #[serde(rename = "handle")]
    pub handle: String,
    /// Framework Icon URL
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    /// Custom Framework ID
    #[serde(rename = "id")]
    pub id: String,
    /// Framework Name
    #[serde(rename = "name")]
    pub name: String,
    /// Org ID
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Framework Update Date
    #[serde(rename = "updated_at")]
    pub updated_at: Option<i64>,
    /// Framework Version
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomFrameworkMetadata {
    pub fn new(
        handle: String,
        id: String,
        name: String,
        org_id: i64,
        version: String,
    ) -> CustomFrameworkMetadata {
        CustomFrameworkMetadata {
            created_at: None,
            created_by: None,
            description: None,
            handle,
            icon_url: None,
            id,
            name,
            org_id,
            updated_at: None,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn icon_url(mut self, value: String) -> Self {
        self.icon_url = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
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

impl<'de> Deserialize<'de> for CustomFrameworkMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomFrameworkMetadataVisitor;
        impl<'a> Visitor<'a> for CustomFrameworkMetadataVisitor {
            type Value = CustomFrameworkMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut icon_url: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut updated_at: Option<i64> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon_url" => {
                            if v.is_null() {
                                continue;
                            }
                            icon_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = CustomFrameworkMetadata {
                    created_at,
                    created_by,
                    description,
                    handle,
                    icon_url,
                    id,
                    name,
                    org_id,
                    updated_at,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomFrameworkMetadataVisitor)
    }
}
