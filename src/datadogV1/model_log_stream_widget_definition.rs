// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogStreamWidgetDefinition {
    /// Which columns to display on the widget.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Vec<String>,
    /// An array of index names to query in the stream. Use [] to query all indexes at once.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Vec<String>,
    /// ID of the log set to use.
    #[serde(rename = "logset", skip_serializing_if = "Option::is_none")]
    pub logset: String,
    /// Amount of log lines to display
    #[serde(rename = "message_display", skip_serializing_if = "Option::is_none")]
    pub message_display: WidgetMessageDisplay,
    /// Query to filter the log stream with.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Whether to show the date column or not
    #[serde(rename = "show_date_column", skip_serializing_if = "Option::is_none")]
    pub show_date_column: bool,
    /// Whether to show the message column or not
    #[serde(rename = "show_message_column", skip_serializing_if = "Option::is_none")]
    pub show_message_column: bool,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: WidgetFieldSort,
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
    /// Type of the log stream widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogStreamWidgetDefinitionType,
}

