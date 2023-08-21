// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FreeTextWidgetDefinition {
    /// Color of the text.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: String,
    /// Size of the text.
    #[serde(rename = "font_size", skip_serializing_if = "Option::is_none")]
    pub font_size: String,
    /// Text to display.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: String,
    /// How to align the text on the widget.
    #[serde(rename = "text_align", skip_serializing_if = "Option::is_none")]
    pub text_align: WidgetTextAlign,
    /// Type of the free text widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: FreeTextWidgetDefinitionType,
}

