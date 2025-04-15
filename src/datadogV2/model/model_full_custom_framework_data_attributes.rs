// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Full Framework Data Attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FullCustomFrameworkDataAttributes {
    /// Creation Timestamp
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Creator
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Framework Description
    #[serde(rename = "description")]
    pub description: String,
    /// Framework Handle
    #[serde(rename = "handle")]
    pub handle: String,
    /// Framework Icon URL
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    /// Framework ID
    #[serde(rename = "id")]
    pub id: String,
    /// Modification Timestamp
    #[serde(rename = "modified_at")]
    pub modified_at: i64,
    /// Framework Name
    #[serde(rename = "name")]
    pub name: String,
    /// Organization ID
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Framework Requirements
    #[serde(rename = "requirements")]
    pub requirements: Vec<crate::datadogV2::model::CustomFrameworkRequirement>,
    /// Framework Version
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FullCustomFrameworkDataAttributes {
    pub fn new(
        created_at: i64,
        created_by: String,
        description: String,
        handle: String,
        icon_url: String,
        id: String,
        modified_at: i64,
        name: String,
        org_id: i64,
        requirements: Vec<crate::datadogV2::model::CustomFrameworkRequirement>,
        version: String,
    ) -> FullCustomFrameworkDataAttributes {
        FullCustomFrameworkDataAttributes {
            created_at,
            created_by,
            description,
            handle,
            icon_url,
            id,
            modified_at,
            name,
            org_id,
            requirements,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for FullCustomFrameworkDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FullCustomFrameworkDataAttributesVisitor;
        impl<'a> Visitor<'a> for FullCustomFrameworkDataAttributesVisitor {
            type Value = FullCustomFrameworkDataAttributes;

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
                let mut modified_at: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut requirements: Option<
                    Vec<crate::datadogV2::model::CustomFrameworkRequirement>,
                > = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon_url" => {
                            icon_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requirements" => {
                            requirements =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let icon_url = icon_url.ok_or_else(|| M::Error::missing_field("icon_url"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let requirements =
                    requirements.ok_or_else(|| M::Error::missing_field("requirements"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = FullCustomFrameworkDataAttributes {
                    created_at,
                    created_by,
                    description,
                    handle,
                    icon_url,
                    id,
                    modified_at,
                    name,
                    org_id,
                    requirements,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FullCustomFrameworkDataAttributesVisitor)
    }
}
