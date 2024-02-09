// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Distribution visualization is another way of showing metrics
/// aggregated across one or several tags, such as hosts.
/// Unlike the heat map, a distribution graph’s x-axis is quantity rather than time.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetDefinition {
    /// A list of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// (Deprecated) The widget legend was replaced by a tooltip and sidebar.
    #[deprecated]
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// List of markers.
    #[serde(rename = "markers")]
    pub markers: Option<Vec<crate::datadogV1::model::WidgetMarker>>,
    /// Array of one request object to display in the widget.
    ///
    /// See the dedicated [Request JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/request_json>)
    ///  to learn how to build the `REQUEST_SCHEMA`.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::DistributionWidgetRequest>,
    /// (Deprecated) The widget legend was replaced by a tooltip and sidebar.
    #[deprecated]
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
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
    /// Type of the distribution widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::DistributionWidgetDefinitionType,
    /// X Axis controls for the distribution widget.
    #[serde(rename = "xaxis")]
    pub xaxis: Option<crate::datadogV1::model::DistributionWidgetXAxis>,
    /// Y Axis controls for the distribution widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::DistributionWidgetYAxis>,
}

impl DistributionWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::DistributionWidgetRequest>,
        type_: crate::datadogV1::model::DistributionWidgetDefinitionType,
    ) -> DistributionWidgetDefinition {
        #[allow(deprecated)]
        DistributionWidgetDefinition {
            custom_links: None,
            legend_size: None,
            markers: None,
            requests,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            xaxis: None,
            yaxis: None,
        }
    }

    #[allow(deprecated)]
    pub fn custom_links(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetCustomLink>,
    ) -> &mut Self {
        self.custom_links = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn legend_size(&mut self, value: String) -> &mut Self {
        self.legend_size = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn markers(&mut self, value: Vec<crate::datadogV1::model::WidgetMarker>) -> &mut Self {
        self.markers = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_legend(&mut self, value: bool) -> &mut Self {
        self.show_legend = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_align(&mut self, value: crate::datadogV1::model::WidgetTextAlign) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn xaxis(&mut self, value: crate::datadogV1::model::DistributionWidgetXAxis) -> &mut Self {
        self.xaxis = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn yaxis(&mut self, value: crate::datadogV1::model::DistributionWidgetYAxis) -> &mut Self {
        self.yaxis = Some(value);
        self
    }
}
