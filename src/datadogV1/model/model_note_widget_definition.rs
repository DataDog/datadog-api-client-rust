// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The notes and links widget is similar to free text widget, but allows for more formatting options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NoteWidgetDefinition {
    /// Background color of the note.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// Content of the note.
    #[serde(rename = "content")]
    pub content: String,
    /// Size of the text.
    #[serde(rename = "font_size")]
    pub font_size: Option<String>,
    /// Whether to add padding or not.
    #[serde(rename = "has_padding")]
    pub has_padding: Option<bool>,
    /// Whether to show a tick or not.
    #[serde(rename = "show_tick")]
    pub show_tick: Option<bool>,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Define how you want to align the text on the widget.
    #[serde(rename = "tick_edge")]
    pub tick_edge: Option<crate::datadogV1::model::WidgetTickEdge>,
    /// Where to position the tick on an edge.
    #[serde(rename = "tick_pos")]
    pub tick_pos: Option<String>,
    /// Type of the note widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NoteWidgetDefinitionType,
    /// Vertical alignment.
    #[serde(rename = "vertical_align")]
    pub vertical_align: Option<crate::datadogV1::model::WidgetVerticalAlign>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NoteWidgetDefinition {
    pub fn new(
        content: String,
        type_: crate::datadogV1::model::NoteWidgetDefinitionType,
    ) -> NoteWidgetDefinition {
        NoteWidgetDefinition {
            background_color: None,
            content,
            font_size: None,
            has_padding: None,
            show_tick: None,
            text_align: None,
            tick_edge: None,
            tick_pos: None,
            type_,
            vertical_align: None,
            _unparsed: false,
        }
    }

    pub fn background_color(mut self, value: String) -> Self {
        self.background_color = Some(value);
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

    pub fn show_tick(mut self, value: bool) -> Self {
        self.show_tick = Some(value);
        self
    }

    pub fn text_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }

    pub fn tick_edge(mut self, value: crate::datadogV1::model::WidgetTickEdge) -> Self {
        self.tick_edge = Some(value);
        self
    }

    pub fn tick_pos(mut self, value: String) -> Self {
        self.tick_pos = Some(value);
        self
    }

    pub fn vertical_align(mut self, value: crate::datadogV1::model::WidgetVerticalAlign) -> Self {
        self.vertical_align = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NoteWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NoteWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for NoteWidgetDefinitionVisitor {
            type Value = NoteWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut background_color: Option<String> = None;
                let mut content: Option<String> = None;
                let mut font_size: Option<String> = None;
                let mut has_padding: Option<bool> = None;
                let mut show_tick: Option<bool> = None;
                let mut text_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut tick_edge: Option<crate::datadogV1::model::WidgetTickEdge> = None;
                let mut tick_pos: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::NoteWidgetDefinitionType> = None;
                let mut vertical_align: Option<crate::datadogV1::model::WidgetVerticalAlign> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "background_color" => {
                            if v.is_null() {
                                continue;
                            }
                            background_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "font_size" => {
                            if v.is_null() {
                                continue;
                            }
                            font_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_padding" => {
                            if v.is_null() {
                                continue;
                            }
                            has_padding =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_tick" => {
                            if v.is_null() {
                                continue;
                            }
                            show_tick = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text_align" => {
                            if v.is_null() {
                                continue;
                            }
                            text_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _text_align) = text_align {
                                match _text_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _text_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tick_edge" => {
                            if v.is_null() {
                                continue;
                            }
                            tick_edge = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _tick_edge) = tick_edge {
                                match _tick_edge {
                                    crate::datadogV1::model::WidgetTickEdge::UnparsedObject(
                                        _tick_edge,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tick_pos" => {
                            if v.is_null() {
                                continue;
                            }
                            tick_pos = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::NoteWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "vertical_align" => {
                            if v.is_null() {
                                continue;
                            }
                            vertical_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _vertical_align) = vertical_align {
                                match _vertical_align {
                                    crate::datadogV1::model::WidgetVerticalAlign::UnparsedObject(_vertical_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NoteWidgetDefinition {
                    background_color,
                    content,
                    font_size,
                    has_padding,
                    show_tick,
                    text_align,
                    tick_edge,
                    tick_pos,
                    type_,
                    vertical_align,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NoteWidgetDefinitionVisitor)
    }
}
