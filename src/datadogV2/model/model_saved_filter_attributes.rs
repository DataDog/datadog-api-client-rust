// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Saved filter details, including the full targeting rules and conditions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SavedFilterAttributes {
    /// The time the saved filter was archived. Omitted when the filter is not archived.
    #[serde(rename = "archived_at")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The time the saved filter was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The ID of the user who created the saved filter.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// How the saved filter was authored. RULES indicates rule-based authoring and LIST indicates list import. Both types return targeting rules and conditions.
    #[serde(rename = "creation_type")]
    pub creation_type: crate::datadogV2::model::SavedFilterCreationType,
    /// A description of the saved filter. Omitted when no description is set.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Optional JSON metadata supplied by migration tools for tracking the source of a saved filter.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// The name of the saved filter.
    #[serde(rename = "name")]
    pub name: String,
    /// Targeting rules combined with OR. Conditions within each rule are combined with AND. Both saved lists and saved rules use this structure.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Vec<crate::datadogV2::model::SavedFilterTargetingRule>,
    /// The time the saved filter was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SavedFilterAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        creation_type: crate::datadogV2::model::SavedFilterCreationType,
        name: String,
        targeting_rules: Vec<crate::datadogV2::model::SavedFilterTargetingRule>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> SavedFilterAttributes {
        SavedFilterAttributes {
            archived_at: None,
            created_at,
            created_by,
            creation_type,
            description: None,
            migration_metadata: None,
            name,
            targeting_rules,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn archived_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
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

impl<'de> Deserialize<'de> for SavedFilterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SavedFilterAttributesVisitor;
        impl<'a> Visitor<'a> for SavedFilterAttributesVisitor {
            type Value = SavedFilterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut creation_type: Option<crate::datadogV2::model::SavedFilterCreationType> =
                    None;
                let mut description: Option<String> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut targeting_rules: Option<
                    Vec<crate::datadogV2::model::SavedFilterTargetingRule>,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived_at" => {
                            if v.is_null() {
                                continue;
                            }
                            archived_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_type" => {
                            creation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _creation_type) = creation_type {
                                match _creation_type {
                                    crate::datadogV2::model::SavedFilterCreationType::UnparsedObject(_creation_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migration_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            targeting_rules =
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
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let creation_type =
                    creation_type.ok_or_else(|| M::Error::missing_field("creation_type"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let targeting_rules =
                    targeting_rules.ok_or_else(|| M::Error::missing_field("targeting_rules"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = SavedFilterAttributes {
                    archived_at,
                    created_at,
                    created_by,
                    creation_type,
                    description,
                    migration_metadata,
                    name,
                    targeting_rules,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SavedFilterAttributesVisitor)
    }
}
