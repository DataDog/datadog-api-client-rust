// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Free text is a widget that allows you to add headings to your screenboard. Commonly used to state the overall purpose of the dashboard. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FreeTextWidgetDefinition {
    /// Color of the text.
    #[serde(rename = "color")]
    pub color: Option<String>,
    /// Size of the text.
    #[serde(rename = "font_size")]
    pub font_size: Option<String>,
    /// Text to display.
    #[serde(rename = "text")]
    pub text: String,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Type of the free text widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::FreeTextWidgetDefinitionType,
}

impl FreeTextWidgetDefinition {
    pub fn new(
        text: String,
        type_: crate::datadogV1::model::FreeTextWidgetDefinitionType,
    ) -> FreeTextWidgetDefinition {
        FreeTextWidgetDefinition {
            color: None,
            font_size: None,
            text,
            text_align: None,
            type_,
        }
    }

    pub fn color(mut self, value: String) -> Self {
        self.color = Some(value);
        self
    }

    pub fn font_size(mut self, value: String) -> Self {
        self.font_size = Some(value);
        self
    }

    pub fn text_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }
}
