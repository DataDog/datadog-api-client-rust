// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOWidgetDefinition {
    /// Additional filters applied to the SLO query.
    #[serde(rename = "additional_query_filters", skip_serializing_if = "Option::is_none")]
    pub additional_query_filters: String,
    /// Defined global time target.
    #[serde(rename = "global_time_target", skip_serializing_if = "Option::is_none")]
    pub global_time_target: String,
    /// Defined error budget.
    #[serde(rename = "show_error_budget", skip_serializing_if = "Option::is_none")]
    pub show_error_budget: bool,
    /// ID of the SLO displayed.
    #[serde(rename = "slo_id", skip_serializing_if = "Option::is_none")]
    pub slo_id: String,
    /// Times being monitored.
    #[serde(rename = "time_windows", skip_serializing_if = "Option::is_none")]
    pub time_windows: Vec<WidgetTimeWindows>,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the SLO widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SLOWidgetDefinitionType,
    /// Define how you want the SLO to be displayed.
    #[serde(rename = "view_mode", skip_serializing_if = "Option::is_none")]
    pub view_mode: WidgetViewMode,
    /// Type of view displayed by the widget.
    #[serde(rename = "view_type", skip_serializing_if = "Option::is_none")]
    pub view_type: String,
}

