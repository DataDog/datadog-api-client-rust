// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterPlotWidgetDefinition {
    /// List of groups used for colors.
    #[serde(rename = "color_by_groups", skip_serializing_if = "Option::is_none")]
    pub color_by_groups: Vec<String>,
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// Widget definition.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: ScatterPlotWidgetDefinitionRequests,
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
    /// Type of the scatter plot widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ScatterPlotWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "xaxis", skip_serializing_if = "Option::is_none")]
    pub xaxis: WidgetAxis,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: WidgetAxis,
}

