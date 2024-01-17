// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The event stream is a widget version of the stream of events
/// on the Event Stream view. Only available on FREE layout dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventStreamWidgetDefinition {
    /// Size to use to display an event.
    #[serde(rename = "event_size")]
    pub event_size: Option<crate::datadogV1::model::WidgetEventSize>,
    /// Query to filter the event stream with.
    #[serde(rename = "query")]
    pub query: String,
    /// The execution method for multi-value filters. Can be either and or or.
    #[serde(rename = "tags_execution")]
    pub tags_execution: Option<String>,
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
    /// Type of the event stream widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::EventStreamWidgetDefinitionType,
}

impl EventStreamWidgetDefinition {
    pub fn new(
        query: String,
        type_: crate::datadogV1::model::EventStreamWidgetDefinitionType,
    ) -> EventStreamWidgetDefinition {
        EventStreamWidgetDefinition {
            event_size: None,
            query,
            tags_execution: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}
