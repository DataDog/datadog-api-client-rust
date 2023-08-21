// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupWidgetDefinition {
    /// Background color of the group title.
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: String,
    /// URL of image to display as a banner for the group.
    #[serde(rename = "banner_img", skip_serializing_if = "Option::is_none")]
    pub banner_img: String,
    /// Layout type of the group.
    #[serde(rename = "layout_type", skip_serializing_if = "Option::is_none")]
    pub layout_type: WidgetLayoutType,
    /// Whether to show the title or not.
    #[serde(rename = "show_title", skip_serializing_if = "Option::is_none")]
    pub show_title: bool,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Type of the group widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: GroupWidgetDefinitionType,
    /// List of widget groups.
    #[serde(rename = "widgets", skip_serializing_if = "Option::is_none")]
    pub widgets: Vec<Widget>,
}

