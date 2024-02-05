// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This widget displays a map of a service to all of the services that call it, and all of the services that it calls.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    /// The ID of the service you want to map.
    #[serde(rename = "service")]
    pub service: String,
    /// The title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the service map widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ServiceMapWidgetDefinitionType,
}

impl ServiceMapWidgetDefinition {
    pub fn new(
        filters: Vec<String>,
        service: String,
        type_: crate::datadogV1::model::ServiceMapWidgetDefinitionType,
    ) -> ServiceMapWidgetDefinition {
        ServiceMapWidgetDefinition {
            custom_links: None,
            filters,
            service,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    pub fn custom_links(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetCustomLink>,
    ) -> &mut Self {
        self.custom_links = Some(value);
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
