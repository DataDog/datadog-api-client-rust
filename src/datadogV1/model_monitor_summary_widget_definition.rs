// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSummaryWidgetDefinition {
    /// Which color to use on the widget.
    #[serde(rename = "color_preference", skip_serializing_if = "Option::is_none")]
    pub color_preference: WidgetColorPreference,
    /// The number of monitors to display.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i64,
    /// What to display on the widget.
    #[serde(rename = "display_format", skip_serializing_if = "Option::is_none")]
    pub display_format: WidgetMonitorSummaryDisplayFormat,
    /// Whether to show counts of 0 or not.
    #[serde(rename = "hide_zero_counts", skip_serializing_if = "Option::is_none")]
    pub hide_zero_counts: bool,
    /// Query to filter the monitors with.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Whether to show the time that has elapsed since the monitor/group triggered.
    #[serde(rename = "show_last_triggered", skip_serializing_if = "Option::is_none")]
    pub show_last_triggered: bool,
    /// Whether to show the priorities column.
    #[serde(rename = "show_priority", skip_serializing_if = "Option::is_none")]
    pub show_priority: bool,
    /// Widget sorting methods.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: WidgetMonitorSummarySort,
    /// The start of the list. Typically 0.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: i64,
    /// Which summary type should be used.
    #[serde(rename = "summary_type", skip_serializing_if = "Option::is_none")]
    pub summary_type: WidgetSummaryType,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the monitor summary widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MonitorSummaryWidgetDefinitionType,
}

