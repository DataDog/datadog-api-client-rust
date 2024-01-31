// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Change graph shows you the change in a value over the time period chosen.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Array of one request object to display in the widget.
    ///
    /// See the dedicated [Request JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/request_json>)
    ///  to learn how to build the `REQUEST_SCHEMA`.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::ChangeWidgetRequest>,
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
    /// Type of the change widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ChangeWidgetDefinitionType,
}

impl ChangeWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::ChangeWidgetRequest>,
        type_: crate::datadogV1::model::ChangeWidgetDefinitionType,
    ) -> ChangeWidgetDefinition {
        ChangeWidgetDefinition {
            custom_links: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    pub fn with_custom_links(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetCustomLink>,
    ) -> &mut Self {
        self.custom_links = Some(value);
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
