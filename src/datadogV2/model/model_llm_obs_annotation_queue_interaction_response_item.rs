// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single interaction result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationQueueInteractionResponseItem {
    /// Whether this interaction already existed in the queue.
    #[serde(rename = "already_existed")]
    pub already_existed: bool,
    /// Identifier of the content for this interaction.
    #[serde(rename = "content_id")]
    pub content_id: String,
    /// Timestamp when the interaction was added to the queue.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Unique identifier of the interaction.
    #[serde(rename = "id")]
    pub id: String,
    /// Timestamp when the interaction was last updated.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Type of interaction in an annotation queue.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsInteractionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationQueueInteractionResponseItem {
    pub fn new(
        already_existed: bool,
        content_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        id: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        type_: crate::datadogV2::model::LLMObsInteractionType,
    ) -> LLMObsAnnotationQueueInteractionResponseItem {
        LLMObsAnnotationQueueInteractionResponseItem {
            already_existed,
            content_id,
            created_at,
            id,
            modified_at,
            type_,
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

impl<'de> Deserialize<'de> for LLMObsAnnotationQueueInteractionResponseItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationQueueInteractionResponseItemVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationQueueInteractionResponseItemVisitor {
            type Value = LLMObsAnnotationQueueInteractionResponseItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut already_existed: Option<bool> = None;
                let mut content_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut type_: Option<crate::datadogV2::model::LLMObsInteractionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "already_existed" => {
                            already_existed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content_id" => {
                            content_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LLMObsInteractionType::UnparsedObject(_type_) => {
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
                let already_existed =
                    already_existed.ok_or_else(|| M::Error::missing_field("already_existed"))?;
                let content_id = content_id.ok_or_else(|| M::Error::missing_field("content_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsAnnotationQueueInteractionResponseItem {
                    already_existed,
                    content_id,
                    created_at,
                    id,
                    modified_at,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationQueueInteractionResponseItemVisitor)
    }
}
