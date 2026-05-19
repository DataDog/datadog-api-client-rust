// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Model Lab project.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabProjectAttributes {
    /// The storage location for project artifacts.
    #[serde(rename = "artifact_storage_location")]
    pub artifact_storage_location: String,
    /// The date and time the project was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The date and time the project was soft-deleted.
    #[serde(
        rename = "deleted_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A description of the project.
    #[serde(rename = "description")]
    pub description: String,
    /// An optional external URL associated with the project.
    #[serde(
        rename = "external_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub external_url: Option<Option<String>>,
    /// Whether the project is starred by the current user.
    #[serde(rename = "is_starred")]
    pub is_starred: bool,
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    /// The UUID of the project owner.
    #[serde(
        rename = "owner_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub owner_id: Option<Option<String>>,
    /// The list of tags associated with the project.
    #[serde(rename = "tags")]
    pub tags: Vec<crate::datadogV2::model::ModelLabTag>,
    /// The date and time the project was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabProjectAttributes {
    pub fn new(
        artifact_storage_location: String,
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        is_starred: bool,
        name: String,
        tags: Vec<crate::datadogV2::model::ModelLabTag>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> ModelLabProjectAttributes {
        ModelLabProjectAttributes {
            artifact_storage_location,
            created_at,
            deleted_at: None,
            description,
            external_url: None,
            is_starred,
            name,
            owner_id: None,
            tags,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deleted_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn external_url(mut self, value: Option<String>) -> Self {
        self.external_url = Some(value);
        self
    }

    pub fn owner_id(mut self, value: Option<String>) -> Self {
        self.owner_id = Some(value);
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

impl<'de> Deserialize<'de> for ModelLabProjectAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabProjectAttributesVisitor;
        impl<'a> Visitor<'a> for ModelLabProjectAttributesVisitor {
            type Value = ModelLabProjectAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut artifact_storage_location: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut description: Option<String> = None;
                let mut external_url: Option<Option<String>> = None;
                let mut is_starred: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut owner_id: Option<Option<String>> = None;
                let mut tags: Option<Vec<crate::datadogV2::model::ModelLabTag>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "artifact_storage_location" => {
                            artifact_storage_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_url" => {
                            external_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_starred" => {
                            is_starred = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner_id" => {
                            owner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let artifact_storage_location = artifact_storage_location
                    .ok_or_else(|| M::Error::missing_field("artifact_storage_location"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let is_starred = is_starred.ok_or_else(|| M::Error::missing_field("is_starred"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = ModelLabProjectAttributes {
                    artifact_storage_location,
                    created_at,
                    deleted_at,
                    description,
                    external_url,
                    is_starred,
                    name,
                    owner_id,
                    tags,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabProjectAttributesVisitor)
    }
}
