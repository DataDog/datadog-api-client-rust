// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertValueWidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id", skip_serializing_if = "Option::is_none")]
    pub alert_id: String,
    /// Number of decimal to show. If not defined, will use the raw value.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: i64,
    /// How to align the text on the widget.
    #[serde(rename = "text_align", skip_serializing_if = "Option::is_none")]
    pub text_align: WidgetTextAlign,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of value in the widget.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the alert value widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: AlertValueWidgetDefinitionType,
    /// Unit to display with the value.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: String,
}

