// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetDefinition {
    /// A list of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// (Deprecated) The widget legend was replaced by a tooltip and sidebar.
    #[serde(rename = "legend_size", skip_serializing_if = "Option::is_none")]
    pub legend_size: String,
    /// List of markers.
    #[serde(rename = "markers", skip_serializing_if = "Option::is_none")]
    pub markers: Vec<WidgetMarker>,
    /// Array of one request object to display in the widget.

See the dedicated [Request JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/request_json)
 to learn how to build the `REQUEST_SCHEMA`.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<DistributionWidgetRequest>,
    /// (Deprecated) The widget legend was replaced by a tooltip and sidebar.
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
    /// Type of the distribution widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: DistributionWidgetDefinitionType,
    /// X Axis controls for the distribution widget.
    #[serde(rename = "xaxis", skip_serializing_if = "Option::is_none")]
    pub xaxis: DistributionWidgetXAxis,
    /// Y Axis controls for the distribution widget.
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: DistributionWidgetYAxis,
}

