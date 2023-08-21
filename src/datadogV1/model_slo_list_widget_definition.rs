// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListWidgetDefinition {
    /// Array of one request object to display in the widget.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<SLOListWidgetRequest>,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the SLO List widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SLOListWidgetDefinitionType,
}

