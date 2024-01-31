// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The funnel visualization displays a funnel of user sessions that maps a sequence of view navigation and user interaction in your application.
///
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunnelWidgetDefinition {
    /// Request payload used to query items.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::FunnelWidgetRequest>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// The title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// The size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of funnel widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::FunnelWidgetDefinitionType,
}

impl FunnelWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::FunnelWidgetRequest>,
        type_: crate::datadogV1::model::FunnelWidgetDefinitionType,
    ) -> FunnelWidgetDefinition {
        FunnelWidgetDefinition {
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
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
