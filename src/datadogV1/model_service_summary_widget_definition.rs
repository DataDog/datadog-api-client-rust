// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSummaryWidgetDefinition {
    /// Number of columns to display.
    #[serde(rename = "display_format", skip_serializing_if = "Option::is_none")]
    pub display_format: WidgetServiceSummaryDisplayFormat,
    /// APM environment.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: String,
    /// APM service.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// Whether to show the latency breakdown or not.
    #[serde(rename = "show_breakdown", skip_serializing_if = "Option::is_none")]
    pub show_breakdown: bool,
    /// Whether to show the latency distribution or not.
    #[serde(rename = "show_distribution", skip_serializing_if = "Option::is_none")]
    pub show_distribution: bool,
    /// Whether to show the error metrics or not.
    #[serde(rename = "show_errors", skip_serializing_if = "Option::is_none")]
    pub show_errors: bool,
    /// Whether to show the hits metrics or not.
    #[serde(rename = "show_hits", skip_serializing_if = "Option::is_none")]
    pub show_hits: bool,
    /// Whether to show the latency metrics or not.
    #[serde(rename = "show_latency", skip_serializing_if = "Option::is_none")]
    pub show_latency: bool,
    /// Whether to show the resource list or not.
    #[serde(rename = "show_resource_list", skip_serializing_if = "Option::is_none")]
    pub show_resource_list: bool,
    /// Size of the widget.
    #[serde(rename = "size_format", skip_serializing_if = "Option::is_none")]
    pub size_format: WidgetSizeFormat,
    /// APM span name.
    #[serde(rename = "span_name", skip_serializing_if = "Option::is_none")]
    pub span_name: String,
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
    /// Type of the service summary widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ServiceSummaryWidgetDefinitionType,
}

