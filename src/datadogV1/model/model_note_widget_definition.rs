// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The notes and links widget is similar to free text widget, but allows for more formatting options.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }
}