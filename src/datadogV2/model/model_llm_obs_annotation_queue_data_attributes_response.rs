// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability annotation queue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationQueueDataAttributesResponse {
    /// Schema defining the labels for an annotation queue.
    #[serde(rename = "annotation_schema")]
    pub annotation_schema: Option<crate::datadogV2::model::LLMObsAnnotationSchema>,
    /// Timestamp when the queue was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the user who created the queue.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Description of the annotation queue.
    #[serde(rename = "description")]
    pub description: String,
    /// Timestamp when the queue was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the user who last modified the queue.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    /// Name of the annotation queue.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier of the user who owns the queue.
    #[serde(rename = "owned_by")]
    pub owned_by: String,
    /// Identifier of the project this queue belongs to.
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationQueueDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        description: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: String,
        name: String,
        owned_by: String,
        project_id: String,
    ) -> LLMObsAnnotationQueueDataAttributesResponse {
        LLMObsAnnotationQueueDataAttributesResponse {
            annotation_schema: None,
            created_at,
            created_by,
            description,
            modified_at,
            modified_by,
            name,
            owned_by,
            project_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn annotation_schema(
        mut self,
        value: crate::datadogV2::model::LLMObsAnnotationSchema,
    ) -> Self {
        self.annotation_schema = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotationQueueDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationQueueDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationQueueDataAttributesResponseVisitor {
            type Value = LLMObsAnnotationQueueDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotation_schema: Option<crate::datadogV2::model::LLMObsAnnotationSchema> =
                    None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut owned_by: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotation_schema" => {
                            if v.is_null() {
                                continue;
                            }
                            annotation_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owned_by" => {
                            owned_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let owned_by = owned_by.ok_or_else(|| M::Error::missing_field("owned_by"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;

                let content = LLMObsAnnotationQueueDataAttributesResponse {
                    annotation_schema,
                    created_at,
                    created_by,
                    description,
                    modified_at,
                    modified_by,
                    name,
                    owned_by,
                    project_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationQueueDataAttributesResponseVisitor)
    }
}
