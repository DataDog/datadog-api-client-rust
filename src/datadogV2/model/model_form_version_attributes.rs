// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a form version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormVersionAttributes {
    /// The time at which the version was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A JSON Schema definition that describes the form's data fields.
    #[serde(rename = "data_definition")]
    pub data_definition: crate::datadogV2::model::FormDataDefinition,
    /// The signature of the version definition.
    #[serde(rename = "definition_signature")]
    pub definition_signature: String,
    /// The ETag for optimistic concurrency control.
    #[serialize_always]
    #[serde(rename = "etag")]
    pub etag: Option<String>,
    /// Whether this version number has ever appeared in the form's publication history.
    #[serde(rename = "has_ever_been_published")]
    pub has_ever_been_published: Option<bool>,
    /// The ID of the form version.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The time at which the version was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The state of a form version.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::FormVersionState,
    /// UI configuration for rendering form fields, including widget overrides, field ordering, and themes.
    #[serde(rename = "ui_definition")]
    pub ui_definition: crate::datadogV2::model::FormUiDefinition,
    /// The ID of the user who created this version.
    #[serde(rename = "user_id")]
    pub user_id: i64,
    /// The UUID of the user who created this version.
    #[serde(rename = "user_uuid")]
    pub user_uuid: uuid::Uuid,
    /// The sequential version number.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormVersionAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        data_definition: crate::datadogV2::model::FormDataDefinition,
        definition_signature: String,
        etag: Option<String>,
        modified_at: chrono::DateTime<chrono::Utc>,
        state: crate::datadogV2::model::FormVersionState,
        ui_definition: crate::datadogV2::model::FormUiDefinition,
        user_id: i64,
        user_uuid: uuid::Uuid,
        version: i64,
    ) -> FormVersionAttributes {
        FormVersionAttributes {
            created_at,
            data_definition,
            definition_signature,
            etag,
            has_ever_been_published: None,
            id: None,
            modified_at,
            state,
            ui_definition,
            user_id,
            user_uuid,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn has_ever_been_published(mut self, value: bool) -> Self {
        self.has_ever_been_published = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for FormVersionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormVersionAttributesVisitor;
        impl<'a> Visitor<'a> for FormVersionAttributesVisitor {
            type Value = FormVersionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut data_definition: Option<crate::datadogV2::model::FormDataDefinition> = None;
                let mut definition_signature: Option<String> = None;
                let mut etag: Option<Option<String>> = None;
                let mut has_ever_been_published: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut state: Option<crate::datadogV2::model::FormVersionState> = None;
                let mut ui_definition: Option<crate::datadogV2::model::FormUiDefinition> = None;
                let mut user_id: Option<i64> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
                let mut version: Option<i64> = None;
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
                        "data_definition" => {
                            data_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "definition_signature" => {
                            definition_signature =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "etag" => {
                            etag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_ever_been_published" => {
                            if v.is_null() {
                                continue;
                            }
                            has_ever_been_published =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::FormVersionState::UnparsedObject(
                                        _state,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "ui_definition" => {
                            ui_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuid" => {
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let data_definition =
                    data_definition.ok_or_else(|| M::Error::missing_field("data_definition"))?;
                let definition_signature = definition_signature
                    .ok_or_else(|| M::Error::missing_field("definition_signature"))?;
                let etag = etag.ok_or_else(|| M::Error::missing_field("etag"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;
                let ui_definition =
                    ui_definition.ok_or_else(|| M::Error::missing_field("ui_definition"))?;
                let user_id = user_id.ok_or_else(|| M::Error::missing_field("user_id"))?;
                let user_uuid = user_uuid.ok_or_else(|| M::Error::missing_field("user_uuid"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = FormVersionAttributes {
                    created_at,
                    data_definition,
                    definition_signature,
                    etag,
                    has_ever_been_published,
                    id,
                    modified_at,
                    state,
                    ui_definition,
                    user_id,
                    user_uuid,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormVersionAttributesVisitor)
    }
}
