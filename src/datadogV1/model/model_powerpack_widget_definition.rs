// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The powerpack widget allows you to keep similar graphs together on your timeboard. Each group has a custom header, can hold one to many graphs, and is collapsible.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackWidgetDefinition {
    /// Background color of the powerpack title.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// URL of image to display as a banner for the powerpack.
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    /// UUID of the associated powerpack.
    #[serde(rename = "powerpack_id")]
    pub powerpack_id: String,
    /// Whether to show the title or not.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Powerpack template variables.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<crate::datadogV1::model::PowerpackTemplateVariables>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the powerpack widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::PowerpackWidgetDefinitionType,
}

impl PowerpackWidgetDefinition {
    pub fn new(
        powerpack_id: String,
        type_: crate::datadogV1::model::PowerpackWidgetDefinitionType,
    ) -> PowerpackWidgetDefinition {
        PowerpackWidgetDefinition {
            background_color: None,
            banner_img: None,
            powerpack_id,
            show_title: None,
            template_variables: None,
            title: None,
            type_,
        }
    }

    pub fn background_color(&mut self, value: String) -> &mut Self {
        self.background_color = Some(value);
        self
    }

    pub fn banner_img(&mut self, value: String) -> &mut Self {
        self.banner_img = Some(value);
        self
    }

    pub fn show_title(&mut self, value: bool) -> &mut Self {
        self.show_title = Some(value);
        self
    }

    pub fn template_variables(
        &mut self,
        value: crate::datadogV1::model::PowerpackTemplateVariables,
    ) -> &mut Self {
        self.template_variables = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }
}
