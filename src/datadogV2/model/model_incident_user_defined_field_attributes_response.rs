// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident user-defined field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserDefinedFieldAttributesResponse {
    /// The resource type this field is attached to. Always "incidents".
    #[serde(rename = "attached_to")]
    pub attached_to: String,
    /// The section in which the field appears. Use "what_happened" for impact-related fields or "why_it_happened" for root cause fields. When null, the field appears in the Attributes section.
    #[serialize_always]
    #[serde(rename = "category")]
    pub category: Option<crate::datadogV2::model::IncidentUserDefinedFieldCategory>,
    /// The lifecycle stage at which the app prompts users to fill out this field. Cannot be set on required fields.
    #[serialize_always]
    #[serde(rename = "collected")]
    pub collected: Option<crate::datadogV2::model::IncidentUserDefinedFieldCollected>,
    /// Timestamp when the field was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The default value for the field.
    #[serialize_always]
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    /// Timestamp when the field was soft-deleted, or null if not deleted.
    #[serialize_always]
    #[serde(rename = "deleted")]
    pub deleted: Option<chrono::DateTime<chrono::Utc>>,
    /// The human-readable name shown in the UI.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Metadata for autocomplete-type user-defined fields, describing how to populate autocomplete options.
    #[serialize_always]
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::IncidentUserDefinedFieldMetadata>,
    /// Timestamp when the field was last modified.
    #[serialize_always]
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// The unique machine-readable name of the field.
    #[serde(rename = "name")]
    pub name: String,
    /// A decimal string representing the field's display order in the UI.
    #[serialize_always]
    #[serde(rename = "ordinal")]
    pub ordinal: Option<String>,
    /// Reserved for future use. Always null.
    #[serialize_always]
    #[serde(rename = "prerequisite")]
    pub prerequisite: Option<String>,
    /// When true, users must fill out this field on incidents.
    #[serde(rename = "required")]
    pub required: bool,
    /// When true, this field is reserved for system use and cannot be deleted.
    #[serde(rename = "reserved")]
    pub reserved: bool,
    /// Reserved for internal use. Always 0.
    #[serde(rename = "table_id")]
    pub table_id: i64,
    /// For metric tag-type fields only, the metric tag key that powers the autocomplete options.
    #[serialize_always]
    #[serde(rename = "tag_key")]
    pub tag_key: Option<String>,
    /// The data type of the field. 1=dropdown, 2=multiselect, 3=textbox, 4=textarray, 5=metrictag, 6=autocomplete, 7=number, 8=datetime.
    #[serialize_always]
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    /// The list of allowed values for dropdown, multiselect, and autocomplete fields.
    #[serialize_always]
    #[serde(rename = "valid_values")]
    pub valid_values: Option<Vec<crate::datadogV2::model::IncidentUserDefinedFieldValidValue>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserDefinedFieldAttributesResponse {
    pub fn new(
        attached_to: String,
        category: Option<crate::datadogV2::model::IncidentUserDefinedFieldCategory>,
        collected: Option<crate::datadogV2::model::IncidentUserDefinedFieldCollected>,
        created: chrono::DateTime<chrono::Utc>,
        default_value: Option<String>,
        deleted: Option<chrono::DateTime<chrono::Utc>>,
        display_name: String,
        metadata: Option<crate::datadogV2::model::IncidentUserDefinedFieldMetadata>,
        modified: Option<chrono::DateTime<chrono::Utc>>,
        name: String,
        ordinal: Option<String>,
        prerequisite: Option<String>,
        required: bool,
        reserved: bool,
        table_id: i64,
        tag_key: Option<String>,
        type_: Option<i32>,
        valid_values: Option<Vec<crate::datadogV2::model::IncidentUserDefinedFieldValidValue>>,
    ) -> IncidentUserDefinedFieldAttributesResponse {
        IncidentUserDefinedFieldAttributesResponse {
            attached_to,
            category,
            collected,
            created,
            default_value,
            deleted,
            display_name,
            metadata,
            modified,
            name,
            ordinal,
            prerequisite,
            required,
            reserved,
            table_id,
            tag_key,
            type_,
            valid_values,
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

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserDefinedFieldAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentUserDefinedFieldAttributesResponseVisitor {
            type Value = IncidentUserDefinedFieldAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attached_to: Option<String> = None;
                let mut category: Option<
                    Option<crate::datadogV2::model::IncidentUserDefinedFieldCategory>,
                > = None;
                let mut collected: Option<
                    Option<crate::datadogV2::model::IncidentUserDefinedFieldCollected>,
                > = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut default_value: Option<Option<String>> = None;
                let mut deleted: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut display_name: Option<String> = None;
                let mut metadata: Option<
                    Option<crate::datadogV2::model::IncidentUserDefinedFieldMetadata>,
                > = None;
                let mut modified: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut name: Option<String> = None;
                let mut ordinal: Option<Option<String>> = None;
                let mut prerequisite: Option<Option<String>> = None;
                let mut required: Option<bool> = None;
                let mut reserved: Option<bool> = None;
                let mut table_id: Option<i64> = None;
                let mut tag_key: Option<Option<String>> = None;
                let mut type_: Option<Option<i32>> = None;
                let mut valid_values: Option<
                    Option<Vec<crate::datadogV2::model::IncidentUserDefinedFieldValidValue>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attached_to" => {
                            attached_to =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    Some(crate::datadogV2::model::IncidentUserDefinedFieldCategory::UnparsedObject(_category)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "collected" => {
                            collected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _collected) = collected {
                                match _collected {
                                    Some(crate::datadogV2::model::IncidentUserDefinedFieldCollected::UnparsedObject(_collected)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_value" => {
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted" => {
                            deleted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ordinal" => {
                            ordinal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prerequisite" => {
                            prerequisite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reserved" => {
                            reserved = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_id" => {
                            table_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_values" => {
                            valid_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attached_to =
                    attached_to.ok_or_else(|| M::Error::missing_field("attached_to"))?;
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let collected = collected.ok_or_else(|| M::Error::missing_field("collected"))?;
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let default_value =
                    default_value.ok_or_else(|| M::Error::missing_field("default_value"))?;
                let deleted = deleted.ok_or_else(|| M::Error::missing_field("deleted"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let ordinal = ordinal.ok_or_else(|| M::Error::missing_field("ordinal"))?;
                let prerequisite =
                    prerequisite.ok_or_else(|| M::Error::missing_field("prerequisite"))?;
                let required = required.ok_or_else(|| M::Error::missing_field("required"))?;
                let reserved = reserved.ok_or_else(|| M::Error::missing_field("reserved"))?;
                let table_id = table_id.ok_or_else(|| M::Error::missing_field("table_id"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let valid_values =
                    valid_values.ok_or_else(|| M::Error::missing_field("valid_values"))?;

                let content = IncidentUserDefinedFieldAttributesResponse {
                    attached_to,
                    category,
                    collected,
                    created,
                    default_value,
                    deleted,
                    display_name,
                    metadata,
                    modified,
                    name,
                    ordinal,
                    prerequisite,
                    required,
                    reserved,
                    table_id,
                    tag_key,
                    type_,
                    valid_values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserDefinedFieldAttributesResponseVisitor)
    }
}
