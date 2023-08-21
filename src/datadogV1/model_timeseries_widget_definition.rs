// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// List of widget events.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Vec<WidgetEvent>,
    /// Columns displayed in the legend.
    #[serde(rename = "legend_columns", skip_serializing_if = "Option::is_none")]
    pub legend_columns: Vec<TimeseriesWidgetLegendColumn>,
    /// Layout of the legend.
    #[serde(rename = "legend_layout", skip_serializing_if = "Option::is_none")]
    pub legend_layout: TimeseriesWidgetLegendLayout,
    /// Available legend sizes for a widget. Should be one of "0", "2", "4", "8", "16", or "auto".
    #[serde(rename = "legend_size", skip_serializing_if = "Option::is_none")]
    pub legend_size: String,
    /// List of markers.
    #[serde(rename = "markers", skip_serializing_if = "Option::is_none")]
    pub markers: Vec<WidgetMarker>,
    /// List of timeseries widget requests.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<TimeseriesWidgetRequest>,
    /// Axis controls for the widget.
    #[serde(rename = "right_yaxis", skip_serializing_if = "Option::is_none")]
    pub right_yaxis: WidgetAxis,
    /// (screenboard only) Show the legend for this widget.
    #[serde(rename = "show_legend", skip_serializing_if = "Option::is_none")]
    pub show_legend: bool,
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
    /// Type of the timeseries widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: TimeseriesWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: WidgetAxis,
}

