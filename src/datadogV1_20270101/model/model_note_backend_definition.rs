// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NoteBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "content")]
    pub content: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "font_size")]
    pub font_size: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "has_padding")]
    pub has_padding: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_tick")]
    pub show_tick: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionTextAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tick_edge")]
    pub tick_edge: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionTickEdge>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tick_pos")]
    pub tick_pos: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::NoteBackendDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "vertical_align")]
    pub vertical_align: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionVerticalAlign>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl NoteBackendDefinition {
    pub fn new(content: String, type_: crate::datadogV1_20270101::model::NoteBackendDefinitionType) -> NoteBackendDefinition {
        NoteBackendDefinition {
            background_color: None,
            content,
            description: None,
            font_size: None,
            has_padding: None,
            notes: None,
            show_tick: None,
            text_align: None,
            tick_edge: None,
            tick_pos: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            vertical_align: None,
            _unparsed: false,
        }
    }
    
    pub fn background_color(mut self, value: String) -> Self {
        self.background_color = Some(value);
        self
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn font_size(mut self, value: String) -> Self {
        self.font_size = Some(value);
        self
    }
    
    pub fn has_padding(mut self, value: bool) -> Self {
        self.has_padding = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn show_tick(mut self, value: bool) -> Self {
        self.show_tick = Some(value);
        self
    }
    
    pub fn text_align(mut self, value: crate::datadogV1_20270101::model::NoteBackendDefinitionTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }
    
    pub fn tick_edge(mut self, value: crate::datadogV1_20270101::model::NoteBackendDefinitionTickEdge) -> Self {
        self.tick_edge = Some(value);
        self
    }
    
    pub fn tick_pos(mut self, value: String) -> Self {
        self.tick_pos = Some(value);
        self
    }
    
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
    
    pub fn title_align(mut self, value: crate::datadogV1_20270101::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }
    
    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
    
    pub fn vertical_align(mut self, value: crate::datadogV1_20270101::model::NoteBackendDefinitionVerticalAlign) -> Self {
        self.vertical_align = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for NoteBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NoteBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for NoteBackendDefinitionVisitor {
            type Value = NoteBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut background_color: Option<String> = None;
                let mut content: Option<String> = None;
                let mut description: Option<String> = None;
                let mut font_size: Option<String> = None;
                let mut has_padding: Option<bool> = None;
                let mut notes: Option<String> = None;
                let mut show_tick: Option<bool> = None;
                let mut text_align: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionTextAlign> = None;
                let mut tick_edge: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionTickEdge> = None;
                let mut tick_pos: Option<String> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionType> = None;
                let mut vertical_align: Option<crate::datadogV1_20270101::model::NoteBackendDefinitionVerticalAlign> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "background_color" => {
                            if v.is_null() {
                                continue;
                            }
                            background_color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "font_size" => {
                            if v.is_null() {
                                continue;
                            }
                            font_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "has_padding" => {
                            if v.is_null() {
                                continue;
                            }
                            has_padding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_tick" => {
                            if v.is_null() {
                                continue;
                            }
                            show_tick = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "text_align" => {
                            if v.is_null() {
                                continue;
                            }
                            text_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _text_align) = text_align {
                                match _text_align {
                                    crate::datadogV1_20270101::model::NoteBackendDefinitionTextAlign::UnparsedObject(_text_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "tick_edge" => {
                            if v.is_null() {
                                continue;
                            }
                            tick_edge = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _tick_edge) = tick_edge {
                                match _tick_edge {
                                    crate::datadogV1_20270101::model::NoteBackendDefinitionTickEdge::UnparsedObject(_tick_edge) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "tick_pos" => {
                            if v.is_null() {
                                continue;
                            }
                            tick_pos = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1_20270101::model::WidgetTextAlign::UnparsedObject(_title_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::NoteBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "vertical_align" => {
                            if v.is_null() {
                                continue;
                            }
                            vertical_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _vertical_align) = vertical_align {
                                match _vertical_align {
                                    crate::datadogV1_20270101::model::NoteBackendDefinitionVerticalAlign::UnparsedObject(_vertical_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NoteBackendDefinition {
                    background_color,
                    content,
                    description,
                    font_size,
                    has_padding,
                    notes,
                    show_tick,
                    text_align,
                    tick_edge,
                    tick_pos,
                    title,
                    title_align,
                    title_size,
                    type_,
                    vertical_align,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NoteBackendDefinitionVisitor)
    }
}
