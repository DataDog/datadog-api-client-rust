// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single content block rendered inside a `display_block` interaction.
/// `type` discriminates which other fields are meaningful:
///
/// - `markdown` / `text`: `content` must be a string.
/// - `header`: `content` must be a string; `level`, when set, must be one of `sm`, `md`, `lg`, `xl`.
/// - `json`: `content` must be a well-formed JSON value (object, array, or scalar).
/// - `image`: `url` is required.
/// - `widget`: `tileDef` is required (any well-formed JSON; the frontend owns the renderable schema).
/// - `llmobs_trace`: `traceId` is required; `interactionType`, when set, must be `trace` or `experiment_trace`.
///
/// `height`, when set, must be positive.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsContentBlock {
    /// Alternative text for an `image` block.
    #[serde(rename = "alt")]
    pub alt: Option<String>,
    /// Block payload. A string for `markdown`, `header`, and `text`; an
    /// arbitrary JSON value (object, array, or scalar) for `json`. Omitted
    /// for `image`, `widget`, and `llmobs_trace`.
    #[serde(rename = "content")]
    pub content: Option<serde_json::Value>,
    /// Optional rendered height. Must be positive when set.
    #[serde(rename = "height")]
    pub height: Option<i64>,
    /// Upstream interaction type referenced by an `llmobs_trace` block.
    /// Restricted to `trace` or `experiment_trace`.
    #[serde(rename = "interactionType")]
    pub interaction_type:
        Option<crate::datadogV2::model::LLMObsContentBlockLLMObsTraceInteractionType>,
    /// Optional label rendered alongside the block.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Visual size for a `header` block.
    #[serde(rename = "level")]
    pub level: Option<crate::datadogV2::model::LLMObsContentBlockHeaderLevel>,
    /// Tile definition for a `widget` block. Required for `widget`. The
    /// schema is owned by the frontend renderer.
    #[serde(rename = "tileDef")]
    pub tile_def: Option<serde_json::Value>,
    /// Unix-millis time range used by chart blocks.
    #[serde(rename = "timeFrame")]
    pub time_frame: Option<crate::datadogV2::model::LLMObsContentBlockTimeFrame>,
    /// Trace identifier. Required for `llmobs_trace` blocks.
    #[serde(rename = "traceId")]
    pub trace_id: Option<String>,
    /// Discriminator for a single `display_block` content block. Adding a
    /// variant requires coordinated changes in the frontend renderer.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LLMObsContentBlockType,
    /// URL of the image. Required for `image` blocks.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsContentBlock {
    pub fn new(type_: crate::datadogV2::model::LLMObsContentBlockType) -> LLMObsContentBlock {
        LLMObsContentBlock {
            alt: None,
            content: None,
            height: None,
            interaction_type: None,
            label: None,
            level: None,
            tile_def: None,
            time_frame: None,
            trace_id: None,
            type_,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alt(mut self, value: String) -> Self {
        self.alt = Some(value);
        self
    }

    pub fn content(mut self, value: serde_json::Value) -> Self {
        self.content = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn interaction_type(
        mut self,
        value: crate::datadogV2::model::LLMObsContentBlockLLMObsTraceInteractionType,
    ) -> Self {
        self.interaction_type = Some(value);
        self
    }

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn level(mut self, value: crate::datadogV2::model::LLMObsContentBlockHeaderLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn tile_def(mut self, value: serde_json::Value) -> Self {
        self.tile_def = Some(value);
        self
    }

    pub fn time_frame(
        mut self,
        value: crate::datadogV2::model::LLMObsContentBlockTimeFrame,
    ) -> Self {
        self.time_frame = Some(value);
        self
    }

    pub fn trace_id(mut self, value: String) -> Self {
        self.trace_id = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsContentBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsContentBlockVisitor;
        impl<'a> Visitor<'a> for LLMObsContentBlockVisitor {
            type Value = LLMObsContentBlock;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alt: Option<String> = None;
                let mut content: Option<serde_json::Value> = None;
                let mut height: Option<i64> = None;
                let mut interaction_type: Option<
                    crate::datadogV2::model::LLMObsContentBlockLLMObsTraceInteractionType,
                > = None;
                let mut label: Option<String> = None;
                let mut level: Option<crate::datadogV2::model::LLMObsContentBlockHeaderLevel> =
                    None;
                let mut tile_def: Option<serde_json::Value> = None;
                let mut time_frame: Option<crate::datadogV2::model::LLMObsContentBlockTimeFrame> =
                    None;
                let mut trace_id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LLMObsContentBlockType> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alt" => {
                            if v.is_null() {
                                continue;
                            }
                            alt = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "height" => {
                            if v.is_null() {
                                continue;
                            }
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interactionType" => {
                            if v.is_null() {
                                continue;
                            }
                            interaction_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _interaction_type) = interaction_type {
                                match _interaction_type {
                                    crate::datadogV2::model::LLMObsContentBlockLLMObsTraceInteractionType::UnparsedObject(_interaction_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "level" => {
                            if v.is_null() {
                                continue;
                            }
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _level) = level {
                                match _level {
                                    crate::datadogV2::model::LLMObsContentBlockHeaderLevel::UnparsedObject(_level) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tileDef" => {
                            if v.is_null() {
                                continue;
                            }
                            tile_def = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeFrame" => {
                            if v.is_null() {
                                continue;
                            }
                            time_frame = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceId" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LLMObsContentBlockType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LLMObsContentBlock {
                    alt,
                    content,
                    height,
                    interaction_type,
                    label,
                    level,
                    tile_def,
                    time_frame,
                    trace_id,
                    type_,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsContentBlockVisitor)
    }
}
