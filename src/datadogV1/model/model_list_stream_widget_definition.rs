// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The list stream visualization displays a table of recent events in your application that
/// match a search criteria using user-defined columns.
///
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamWidgetDefinition {
    /// Available legend sizes for a widget. Should be one of "0", "2", "4", "8", "16", or "auto".
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// Request payload used to query items.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::ListStreamWidgetRequest>,
    /// Whether or not to display the legend on this widget.
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the list stream widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ListStreamWidgetDefinitionType,
}

impl ListStreamWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::ListStreamWidgetRequest>,
        type_: crate::datadogV1::model::ListStreamWidgetDefinitionType,
    ) -> ListStreamWidgetDefinition {
        ListStreamWidgetDefinition {
            legend_size: None,
            requests,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    pub fn with_legend_size(&mut self, value: String) -> &mut Self {
        self.legend_size = Some(value);
        self
    }

    pub fn with_show_legend(&mut self, value: bool) -> &mut Self {
        self.show_legend = Some(value);
        self
    }

    pub fn with_time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    pub fn with_title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn with_title_align(
        &mut self,
        value: crate::datadogV1::model::WidgetTextAlign,
    ) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    pub fn with_title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }
}
