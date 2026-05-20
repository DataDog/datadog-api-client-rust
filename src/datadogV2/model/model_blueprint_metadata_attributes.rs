// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a blueprint metadata resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BlueprintMetadataAttributes {
    /// The timestamp when the blueprint was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A description of what the blueprint does.
    #[serde(rename = "description")]
    pub description: String,
    /// The human-readable name of the blueprint.
    #[serde(rename = "name")]
    pub name: String,
    /// The unique slug identifier of the blueprint.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Tags associated with the blueprint.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The background style of the blueprint tile.
    #[serde(rename = "tile_background")]
    pub tile_background: Option<String>,
    /// The fully qualified name of the action used as the tile icon.
    #[serde(rename = "tile_icon_action_fqn")]
    pub tile_icon_action_fqn: Option<String>,
    /// The timestamp when the blueprint was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BlueprintMetadataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        name: String,
        slug: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> BlueprintMetadataAttributes {
        BlueprintMetadataAttributes {
            created_at,
            description,
            name,
            slug,
            tags: None,
            tile_background: None,
            tile_icon_action_fqn: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn tile_background(mut self, value: String) -> Self {
        self.tile_background = Some(value);
        self
    }

    pub fn tile_icon_action_fqn(mut self, value: String) -> Self {
        self.tile_icon_action_fqn = Some(value);
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

impl<'de> Deserialize<'de> for BlueprintMetadataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlueprintMetadataAttributesVisitor;
        impl<'a> Visitor<'a> for BlueprintMetadataAttributesVisitor {
            type Value = BlueprintMetadataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut slug: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut tile_background: Option<String> = None;
                let mut tile_icon_action_fqn: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slug" => {
                            slug = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tile_background" => {
                            if v.is_null() {
                                continue;
                            }
                            tile_background =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tile_icon_action_fqn" => {
                            if v.is_null() {
                                continue;
                            }
                            tile_icon_action_fqn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let slug = slug.ok_or_else(|| M::Error::missing_field("slug"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = BlueprintMetadataAttributes {
                    created_at,
                    description,
                    name,
                    slug,
                    tags,
                    tile_background,
                    tile_icon_action_fqn,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BlueprintMetadataAttributesVisitor)
    }
}
