// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An interaction whose rendered content is supplied directly as a list
/// of display blocks. The server generates `content_id` deterministically
/// from the block list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDisplayBlockInteractionItem {
    /// List of content blocks that make up a `display_block` interaction.
    /// Must contain at least one block.
    #[serde(rename = "display_block")]
    pub display_block: Vec<crate::datadogV2::model::LLMObsContentBlock>,
    /// Type discriminator for a `display_block` interaction.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsDisplayBlockInteractionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDisplayBlockInteractionItem {
    pub fn new(
        display_block: Vec<crate::datadogV2::model::LLMObsContentBlock>,
        type_: crate::datadogV2::model::LLMObsDisplayBlockInteractionType,
    ) -> LLMObsDisplayBlockInteractionItem {
        LLMObsDisplayBlockInteractionItem {
            display_block,
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

impl<'de> Deserialize<'de> for LLMObsDisplayBlockInteractionItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDisplayBlockInteractionItemVisitor;
        impl<'a> Visitor<'a> for LLMObsDisplayBlockInteractionItemVisitor {
            type Value = LLMObsDisplayBlockInteractionItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_block: Option<Vec<crate::datadogV2::model::LLMObsContentBlock>> =
                    None;
                let mut type_: Option<crate::datadogV2::model::LLMObsDisplayBlockInteractionType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_block" => {
                            display_block =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let display_block =
                    display_block.ok_or_else(|| M::Error::missing_field("display_block"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsDisplayBlockInteractionItem {
                    display_block,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDisplayBlockInteractionItemVisitor)
    }
}
