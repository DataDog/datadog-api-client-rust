// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This visualization displays a series of values by country on a world map.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinition {
    /// A list of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Array of one request object to display in the widget. The request must contain a `group-by` tag whose value is a country ISO code.
    ///
    /// See the [Request JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/request_json>)
    /// for information about building the `REQUEST_SCHEMA`.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::GeomapWidgetRequest>,
    /// The style to apply to the widget.
    #[serde(rename = "style")]
    pub style: crate::datadogV1::model::GeomapWidgetDefinitionStyle,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// The title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// The size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the geomap widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::GeomapWidgetDefinitionType,
    /// The view of the world that the map should render.
    #[serde(rename = "view")]
    pub view: crate::datadogV1::model::GeomapWidgetDefinitionView,
}

impl GeomapWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::GeomapWidgetRequest>,
        style: crate::datadogV1::model::GeomapWidgetDefinitionStyle,
        type_: crate::datadogV1::model::GeomapWidgetDefinitionType,
        view: crate::datadogV1::model::GeomapWidgetDefinitionView,
    ) -> GeomapWidgetDefinition {
        GeomapWidgetDefinition {
            custom_links: None,
            requests,
            style,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            view,
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
