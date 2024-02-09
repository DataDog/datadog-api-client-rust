// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The timeseries visualization allows you to display the evolution of one or more metrics, log events, or Indexed Spans over time.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// List of widget events.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV1::model::WidgetEvent>>,
    /// Columns displayed in the legend.
    #[serde(rename = "legend_columns")]
    pub legend_columns: Option<Vec<crate::datadogV1::model::TimeseriesWidgetLegendColumn>>,
    /// Layout of the legend.
    #[serde(rename = "legend_layout")]
    pub legend_layout: Option<crate::datadogV1::model::TimeseriesWidgetLegendLayout>,
    /// Available legend sizes for a widget. Should be one of "0", "2", "4", "8", "16", or "auto".
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// List of markers.
    #[serde(rename = "markers")]
    pub markers: Option<Vec<crate::datadogV1::model::WidgetMarker>>,
    /// List of timeseries widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::TimeseriesWidgetRequest>,
    /// Axis controls for the widget.
    #[serde(rename = "right_yaxis")]
    pub right_yaxis: Option<crate::datadogV1::model::WidgetAxis>,
    /// (screenboard only) Show the legend for this widget.
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
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
    /// Type of the timeseries widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TimeseriesWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::WidgetAxis>,
}

impl TimeseriesWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::TimeseriesWidgetRequest>,
        type_: crate::datadogV1::model::TimeseriesWidgetDefinitionType,
    ) -> TimeseriesWidgetDefinition {
        TimeseriesWidgetDefinition {
            custom_links: None,
            events: None,
            legend_columns: None,
            legend_layout: None,
            legend_size: None,
            markers: None,
            requests,
            right_yaxis: None,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            yaxis: None,
        }
    }

    pub fn custom_links(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetCustomLink>,
    ) -> &mut Self {
        self.custom_links = Some(value);
        self
    }

    pub fn events(&mut self, value: Vec<crate::datadogV1::model::WidgetEvent>) -> &mut Self {
        self.events = Some(value);
        self
    }

    pub fn legend_columns(
        &mut self,
        value: Vec<crate::datadogV1::model::TimeseriesWidgetLegendColumn>,
    ) -> &mut Self {
        self.legend_columns = Some(value);
        self
    }

    pub fn legend_layout(
        &mut self,
        value: crate::datadogV1::model::TimeseriesWidgetLegendLayout,
    ) -> &mut Self {
        self.legend_layout = Some(value);
        self
    }

    pub fn legend_size(&mut self, value: String) -> &mut Self {
        self.legend_size = Some(value);
        self
    }

    pub fn markers(&mut self, value: Vec<crate::datadogV1::model::WidgetMarker>) -> &mut Self {
        self.markers = Some(value);
        self
    }

    pub fn right_yaxis(&mut self, value: crate::datadogV1::model::WidgetAxis) -> &mut Self {
        self.right_yaxis = Some(value);
        self
    }

    pub fn show_legend(&mut self, value: bool) -> &mut Self {
        self.show_legend = Some(value);
        self
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

    pub fn yaxis(&mut self, value: crate::datadogV1::model::WidgetAxis) -> &mut Self {
        self.yaxis = Some(value);
        self
    }
}
