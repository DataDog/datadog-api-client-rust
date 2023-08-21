// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamWidgetDefinition {
    /// Available legend sizes for a widget. Should be one of "0", "2", "4", "8", "16", or "auto".
    #[serde(rename = "legend_size", skip_serializing_if = "Option::is_none")]
    pub legend_size: String,
    /// Request payload used to query items.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<ListStreamWidgetRequest>,
    /// Whether or not to display the legend on this widget.
    #[serde(rename = "show_legend", skip_serializing_if = "Option::is_none")]
    pub show_legend: bool,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the list stream widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ListStreamWidgetDefinitionType,
}

