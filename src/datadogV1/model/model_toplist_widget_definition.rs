// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The top list visualization enables you to display a list of Tag value like hostname or service with the most or least of any metric value, such as highest consumers of CPU, hosts with the least disk space, etc.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToplistWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// List of top list widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::ToplistWidgetRequest>,
    /// Style customization for a top list widget.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::ToplistWidgetStyle>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the top list widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ToplistWidgetDefinitionType,
}

impl ToplistWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::ToplistWidgetRequest>,
        type_: crate::datadogV1::model::ToplistWidgetDefinitionType,
    ) -> ToplistWidgetDefinition {
        ToplistWidgetDefinition {
            custom_links: None,
            requests,
            style: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    pub fn style(mut self, value: crate::datadogV1::model::ToplistWidgetStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
}
