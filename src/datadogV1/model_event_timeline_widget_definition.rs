// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventTimelineWidgetDefinition {
    /// Query to filter the event timeline with.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The execution method for multi-value filters. Can be either and or or.
    #[serde(rename = "tags_execution", skip_serializing_if = "Option::is_none")]
    pub tags_execution: String,
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
    /// Type of the event timeline widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: EventTimelineWidgetDefinitionType,
}

