// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogStreamWidgetDefinition {
    /// Which columns to display on the widget.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,
    /// An array of index names to query in the stream. Use [] to query all indexes at once.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// ID of the log set to use.
    #[deprecated]
    #[serde(rename = "logset")]
    pub logset: Option<String>,
    /// Amount of log lines to display
    #[serde(rename = "message_display")]
    pub message_display: Option<crate::datadogV1::model::WidgetMessageDisplay>,
    /// Query to filter the log stream with.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Whether to show the date column or not
    #[serde(rename = "show_date_column")]
    pub show_date_column: Option<bool>,
    /// Whether to show the message column or not
    #[serde(rename = "show_message_column")]
    pub show_message_column: Option<bool>,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: Option<Box<crate::datadogV1::model::WidgetFieldSort>>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<Box<crate::datadogV1::model::WidgetTime>>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the log stream widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogStreamWidgetDefinitionType,
}

impl LogStreamWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::LogStreamWidgetDefinitionType,
    ) -> LogStreamWidgetDefinition {
        #[allow(deprecated)]
        LogStreamWidgetDefinition {
            columns: None,
            indexes: None,
            logset: None,
            message_display: None,
            query: None,
            show_date_column: None,
            show_message_column: None,
            sort: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}
