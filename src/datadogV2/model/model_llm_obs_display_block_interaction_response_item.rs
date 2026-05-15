// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A display_block interaction result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDisplayBlockInteractionResponseItem {
    /// Whether this interaction already existed in the queue.
    #[serde(rename = "already_existed")]
    pub already_existed: bool,
    /// Server-generated deterministic identifier derived from the block list.
    #[serde(rename = "content_id")]
    pub content_id: String,
    /// List of content blocks that make up a `display_block` interaction.
    /// Must contain at least one block.
    #[serde(rename = "display_block")]
    pub display_block: Vec<crate::datadogV2::model::LLMObsContentBlock>,
    /// Unique identifier of the interaction.
    #[serde(rename = "id")]
    pub id: String,
    /// Type discriminator for a `display_block` interaction.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsDisplayBlockInteractionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDisplayBlockInteractionResponseItem {
    pub fn new(
        already_existed: bool,
        content_id: String,
        display_block: Vec<crate::datadogV2::model::LLMObsContentBlock>,
        id: String,
        type_: crate::datadogV2::model::LLMObsDisplayBlockInteractionType,
    ) -> LLMObsDisplayBlockInteractionResponseItem {
        LLMObsDisplayBlockInteractionResponseItem {
            already_existed,
            content_id,
            display_block,
            id,
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

impl<'de> Deserialize<'de> for LLMObsDisplayBlockInteractionResponseItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDisplayBlockInteractionResponseItemVisitor;
        impl<'a> Visitor<'a> for LLMObsDisplayBlockInteractionResponseItemVisitor {
            type Value = LLMObsDisplayBlockInteractionResponseItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut already_existed: Option<bool> = None;
                let mut content_id: Option<String> = None;
                let mut display_block: Option<Vec<crate::datadogV2::model::LLMObsContentBlock>> =
                    None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LLMObsDisplayBlockInteractionType> =
                    None;
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
                        "display_block" => {
                            display_block =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LLMObsDisplayBlockInteractionType::UnparsedObject(_type_) => {
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
                let display_block =
                    display_block.ok_or_else(|| M::Error::missing_field("display_block"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsDisplayBlockInteractionResponseItem {
                    already_existed,
                    content_id,
                    display_block,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDisplayBlockInteractionResponseItemVisitor)
    }
}
