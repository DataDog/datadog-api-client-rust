// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// Controls the display of the search bar.
    #[serde(rename = "has_search_bar", skip_serializing_if = "Option::is_none")]
    pub has_search_bar: TableWidgetHasSearchBar,
    /// Widget definition.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<TableWidgetRequest>,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// Title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the table widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: TableWidgetDefinitionType,
}

