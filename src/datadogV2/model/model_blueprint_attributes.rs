// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a blueprint resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BlueprintAttributes {
    /// The timestamp when the blueprint was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The app definition type.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::AppDefinitionType,
    /// A description of what the blueprint does.
    #[serde(rename = "description")]
    pub description: String,
    /// Embedded datastore blueprints.
    #[serde(rename = "embedded_datastore_blueprints")]
    pub embedded_datastore_blueprints:
        Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Embedded native actions.
    #[serde(rename = "embedded_native_actions")]
    pub embedded_native_actions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Embedded workflow blueprints.
    #[serde(rename = "embedded_workflow_blueprints")]
    pub embedded_workflow_blueprints: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The integration ID associated with the blueprint.
    #[serde(rename = "integration_id")]
    pub integration_id: Option<String>,
    /// Mocked outputs for testing the blueprint.
    #[serde(rename = "mocked_outputs")]
    pub mocked_outputs: Option<std::collections::BTreeMap<String, serde_json::Value>>,
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

impl BlueprintAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        definition: crate::datadogV2::model::AppDefinitionType,
        description: String,
        name: String,
        slug: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> BlueprintAttributes {
        BlueprintAttributes {
            created_at,
            definition,
            description,
            embedded_datastore_blueprints: None,
            embedded_native_actions: None,
            embedded_workflow_blueprints: None,
            integration_id: None,
            mocked_outputs: None,
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

    pub fn embedded_datastore_blueprints(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.embedded_datastore_blueprints = Some(value);
        self
    }

    pub fn embedded_native_actions(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.embedded_native_actions = Some(value);
        self
    }

    pub fn embedded_workflow_blueprints(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.embedded_workflow_blueprints = Some(value);
        self
    }

    pub fn integration_id(mut self, value: String) -> Self {
        self.integration_id = Some(value);
        self
    }

    pub fn mocked_outputs(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.mocked_outputs = Some(value);
        self
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

impl<'de> Deserialize<'de> for BlueprintAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlueprintAttributesVisitor;
        impl<'a> Visitor<'a> for BlueprintAttributesVisitor {
            type Value = BlueprintAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut definition: Option<crate::datadogV2::model::AppDefinitionType> = None;
                let mut description: Option<String> = None;
                let mut embedded_datastore_blueprints: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut embedded_native_actions: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut embedded_workflow_blueprints: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut integration_id: Option<String> = None;
                let mut mocked_outputs: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
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
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _definition) = definition {
                                match _definition {
                                    crate::datadogV2::model::AppDefinitionType::UnparsedObject(
                                        _definition,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "embedded_datastore_blueprints" => {
                            if v.is_null() {
                                continue;
                            }
                            embedded_datastore_blueprints =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "embedded_native_actions" => {
                            if v.is_null() {
                                continue;
                            }
                            embedded_native_actions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "embedded_workflow_blueprints" => {
                            if v.is_null() {
                                continue;
                            }
                            embedded_workflow_blueprints =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_id" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mocked_outputs" => {
                            if v.is_null() {
                                continue;
                            }
                            mocked_outputs =
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
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let slug = slug.ok_or_else(|| M::Error::missing_field("slug"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = BlueprintAttributes {
                    created_at,
                    definition,
                    description,
                    embedded_datastore_blueprints,
                    embedded_native_actions,
                    embedded_workflow_blueprints,
                    integration_id,
                    mocked_outputs,
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

        deserializer.deserialize_any(BlueprintAttributesVisitor)
    }
}
