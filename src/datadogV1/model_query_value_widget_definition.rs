// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryValueWidgetDefinition {
    /// Whether to use auto-scaling or not.
    #[serde(rename = "autoscale", skip_serializing_if = "Option::is_none")]
    pub autoscale: bool,
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// Display a unit of your choice on the widget.
    #[serde(rename = "custom_unit", skip_serializing_if = "Option::is_none")]
    pub custom_unit: String,
    /// Number of decimals to show. If not defined, the widget uses the raw value.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: i64,
    /// Widget definition.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<QueryValueWidgetRequest>,
    /// How to align the text on the widget.
    #[serde(rename = "text_align", skip_serializing_if = "Option::is_none")]
    pub text_align: WidgetTextAlign,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// Set a timeseries on the widget background.
    #[serde(rename = "timeseries_background")]
    pub timeseries_background: TimeseriesBackground,
    /// Title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the query value widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: QueryValueWidgetDefinitionType,
}

