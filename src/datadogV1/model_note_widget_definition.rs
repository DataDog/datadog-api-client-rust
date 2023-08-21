// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoteWidgetDefinition {
    /// Background color of the note.
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: String,
    /// Content of the note.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: String,
    /// Size of the text.
    #[serde(rename = "font_size", skip_serializing_if = "Option::is_none")]
    pub font_size: String,
    /// Whether to add padding or not.
    #[serde(rename = "has_padding", skip_serializing_if = "Option::is_none")]
    pub has_padding: bool,
    /// Whether to show a tick or not.
    #[serde(rename = "show_tick", skip_serializing_if = "Option::is_none")]
    pub show_tick: bool,
    /// How to align the text on the widget.
    #[serde(rename = "text_align", skip_serializing_if = "Option::is_none")]
    pub text_align: WidgetTextAlign,
    /// Define how you want to align the text on the widget.
    #[serde(rename = "tick_edge", skip_serializing_if = "Option::is_none")]
    pub tick_edge: WidgetTickEdge,
    /// Where to position the tick on an edge.
    #[serde(rename = "tick_pos", skip_serializing_if = "Option::is_none")]
    pub tick_pos: String,
    /// Type of the note widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: NoteWidgetDefinitionType,
    /// Vertical alignment.
    #[serde(rename = "vertical_align", skip_serializing_if = "Option::is_none")]
    pub vertical_align: WidgetVerticalAlign,
}

