// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An annotated interaction returned by the cross-queue lookup, including the source queue metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotatedInteractionByTraceItem {
    /// List of annotations for this interaction.
    #[serde(rename = "annotations")]
    pub annotations: Vec<crate::datadogV2::model::LLMObsAnnotationItem>,
    /// Upstream entity identifier (trace ID, session ID, or deterministic display_block ID).
    #[serde(rename = "content_id")]
    pub content_id: String,
    /// Timestamp when the interaction was added to the queue.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// List of content blocks that make up a `display_block` interaction.
    /// Must contain at least one block.
    #[serde(rename = "display_block")]
    pub display_block: Option<Vec<crate::datadogV2::model::LLMObsContentBlock>>,
    /// Unique identifier of the interaction.
    #[serde(rename = "id")]
    pub id: String,
    /// Timestamp when the interaction was last updated.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the annotation queue this interaction belongs to.
    #[serde(rename = "queue_id")]
    pub queue_id: String,
    /// Name of the annotation queue this interaction belongs to.
    #[serde(rename = "queue_name")]
    pub queue_name: String,
    /// Type of an annotated interaction.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsAnyInteractionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotatedInteractionByTraceItem {
    pub fn new(
        annotations: Vec<crate::datadogV2::model::LLMObsAnnotationItem>,
        content_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        id: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        queue_id: String,
        queue_name: String,
        type_: crate::datadogV2::model::LLMObsAnyInteractionType,
    ) -> LLMObsAnnotatedInteractionByTraceItem {
        LLMObsAnnotatedInteractionByTraceItem {
            annotations,
            content_id,
            created_at,
            display_block: None,
            id,
            modified_at,
            queue_id,
            queue_name,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_block(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsContentBlock>,
    ) -> Self {
        self.display_block = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotatedInteractionByTraceItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotatedInteractionByTraceItemVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotatedInteractionByTraceItemVisitor {
            type Value = LLMObsAnnotatedInteractionByTraceItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotations: Option<Vec<crate::datadogV2::model::LLMObsAnnotationItem>> =
                    None;
                let mut content_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut display_block: Option<Vec<crate::datadogV2::model::LLMObsContentBlock>> =
                    None;
                let mut id: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut queue_id: Option<String> = None;
                let mut queue_name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LLMObsAnyInteractionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotations" => {
                            annotations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content_id" => {
                            content_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_block" => {
                            if v.is_null() {
                                continue;
                            }
                            display_block =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queue_id" => {
                            queue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queue_name" => {
                            queue_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LLMObsAnyInteractionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotations =
                    annotations.ok_or_else(|| M::Error::missing_field("annotations"))?;
                let content_id = content_id.ok_or_else(|| M::Error::missing_field("content_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let queue_id = queue_id.ok_or_else(|| M::Error::missing_field("queue_id"))?;
                let queue_name = queue_name.ok_or_else(|| M::Error::missing_field("queue_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsAnnotatedInteractionByTraceItem {
                    annotations,
                    content_id,
                    created_at,
                    display_block,
                    id,
                    modified_at,
                    queue_id,
                    queue_name,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotatedInteractionByTraceItemVisitor)
    }
}
