// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Alert graphs are timeseries graphs showing the current status of any monitor defined on your system.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertGraphWidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// The title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the alert graph widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::AlertGraphWidgetDefinitionType,
    /// Whether to display the Alert Graph as a timeseries or a top list.
    #[serde(rename = "viz_type")]
    pub viz_type: crate::datadogV1::model::WidgetVizType,
}

impl AlertGraphWidgetDefinition {
    pub fn new(
        alert_id: String,
        type_: crate::datadogV1::model::AlertGraphWidgetDefinitionType,
        viz_type: crate::datadogV1::model::WidgetVizType,
    ) -> AlertGraphWidgetDefinition {
        AlertGraphWidgetDefinition {
            alert_id,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            viz_type,
        }
    }

    pub fn time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(&mut self, value: crate::datadogV1::model::WidgetTextAlign) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }
}
