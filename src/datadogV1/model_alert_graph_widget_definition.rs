// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertGraphWidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id", skip_serializing_if = "Option::is_none")]
    pub alert_id: String,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// The title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the alert graph widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: AlertGraphWidgetDefinitionType,
    /// Whether to display the Alert Graph as a timeseries or a top list.
    #[serde(rename = "viz_type", skip_serializing_if = "Option::is_none")]
    pub viz_type: WidgetVizType,
}

