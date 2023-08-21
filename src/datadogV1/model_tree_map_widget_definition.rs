// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreeMapWidgetDefinition {
    /// (deprecated) The attribute formerly used to determine color in the widget.
    #[serde(rename = "color_by", skip_serializing_if = "Option::is_none")]
    pub color_by: TreeMapColorBy,
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// (deprecated) The attribute formerly used to group elements in the widget.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: TreeMapGroupBy,
    /// List of treemap widget requests.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<TreeMapWidgetRequest>,
    /// (deprecated) The attribute formerly used to determine size in the widget.
    #[serde(rename = "size_by", skip_serializing_if = "Option::is_none")]
    pub size_by: TreeMapSizeBy,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// Title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// Type of the treemap widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: TreeMapWidgetDefinitionType,
}

