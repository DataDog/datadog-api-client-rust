// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The groups widget allows you to keep similar graphs together on your timeboard. Each group has a custom header, can hold one to many graphs, and is collapsible.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupWidgetDefinition {
    /// Background color of the group title.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// URL of image to display as a banner for the group.
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    /// Layout type of the group.
    #[serde(rename = "layout_type")]
    pub layout_type: crate::datadogV1::model::WidgetLayoutType,
    /// Whether to show the title or not.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Type of the group widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::GroupWidgetDefinitionType,
    /// List of widget groups.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::datadogV1::model::Widget>,
}

impl GroupWidgetDefinition {
    pub fn new(
        layout_type: crate::datadogV1::model::WidgetLayoutType,
        type_: crate::datadogV1::model::GroupWidgetDefinitionType,
        widgets: Vec<crate::datadogV1::model::Widget>,
    ) -> GroupWidgetDefinition {
        GroupWidgetDefinition {
            background_color: None,
            banner_img: None,
            layout_type,
            show_title: None,
            title: None,
            title_align: None,
            type_,
            widgets,
        }
    }

    pub fn background_color(mut self, value: String) -> Self {
        self.background_color = Some(value);
        self
    }

    pub fn banner_img(mut self, value: String) -> Self {
        self.banner_img = Some(value);
        self
    }

    pub fn show_title(mut self, value: bool) -> Self {
        self.show_title = Some(value);
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
}
