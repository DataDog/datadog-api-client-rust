// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single annotation on an interaction.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationItem {
    /// Timestamp when the annotation was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the user who created the annotation.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Unique identifier of the annotation.
    #[serde(rename = "id")]
    pub id: String,
    /// Identifier of the interaction this annotation belongs to.
    #[serde(rename = "interaction_id")]
    pub interaction_id: String,
    /// The label values for this annotation.
    #[serde(rename = "label_values")]
    pub label_values: std::collections::BTreeMap<String, serde_json::Value>,
    /// Timestamp when the annotation was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the user who last modified the annotation.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationItem {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        id: String,
        interaction_id: String,
        label_values: std::collections::BTreeMap<String, serde_json::Value>,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: String,
    ) -> LLMObsAnnotationItem {
        LLMObsAnnotationItem {
            created_at,
            created_by,
            id,
            interaction_id,
            label_values,
            modified_at,
            modified_by,
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

impl<'de> Deserialize<'de> for LLMObsAnnotationItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationItemVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationItemVisitor {
            type Value = LLMObsAnnotationItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut id: Option<String> = None;
                let mut interaction_id: Option<String> = None;
                let mut label_values: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
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
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interaction_id" => {
                            interaction_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label_values" => {
                            label_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
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
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let interaction_id =
                    interaction_id.ok_or_else(|| M::Error::missing_field("interaction_id"))?;
                let label_values =
                    label_values.ok_or_else(|| M::Error::missing_field("label_values"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;

                let content = LLMObsAnnotationItem {
                    created_at,
                    created_by,
                    id,
                    interaction_id,
                    label_values,
                    modified_at,
                    modified_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationItemVisitor)
    }
}
