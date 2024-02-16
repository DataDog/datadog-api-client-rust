// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Alert values are query values showing the current value of the metric in any monitor defined on your system.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertValueWidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
    /// Number of decimal to show. If not defined, will use the raw value.
    #[serde(rename = "precision")]
    pub precision: Option<i64>,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of value in the widget.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the alert value widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::AlertValueWidgetDefinitionType,
    /// Unit to display with the value.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
}

impl AlertValueWidgetDefinition {
    pub fn new(
        alert_id: String,
        type_: crate::datadogV1::model::AlertValueWidgetDefinitionType,
    ) -> AlertValueWidgetDefinition {
        AlertValueWidgetDefinition {
            alert_id,
            precision: None,
            text_align: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            unit: None,
        }
    }

    pub fn precision(&mut self, value: i64) -> &mut Self {
        self.precision = Some(value);
        self
    }

    pub fn text_align(&mut self, value: crate::datadogV1::model::WidgetTextAlign) -> &mut Self {
        self.text_align = Some(value);
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

    pub fn unit(&mut self, value: String) -> &mut Self {
        self.unit = Some(value);
        self
    }
}
