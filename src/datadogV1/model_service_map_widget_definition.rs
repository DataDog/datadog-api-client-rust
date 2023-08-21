// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Vec<String>,
    /// The ID of the service you want to map.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// The title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the service map widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ServiceMapWidgetDefinitionType,
}

