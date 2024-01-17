// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The scatter plot visualization allows you to graph a chosen scope over two different metrics with their respective aggregation.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterPlotWidgetDefinition {
    /// List of groups used for colors.
    #[serde(rename = "color_by_groups")]
    pub color_by_groups: Option<Vec<String>>,
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Widget definition.
    #[serde(rename = "requests")]
    pub requests: Box<crate::datadogV1::model::ScatterPlotWidgetDefinitionRequests>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<Box<crate::datadogV1::model::WidgetTime>>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the scatter plot widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ScatterPlotWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "xaxis")]
    pub xaxis: Option<Box<crate::datadogV1::model::WidgetAxis>>,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<Box<crate::datadogV1::model::WidgetAxis>>,
}

impl ScatterPlotWidgetDefinition {
    pub fn new(
        requests: Box<crate::datadogV1::model::ScatterPlotWidgetDefinitionRequests>,
        type_: crate::datadogV1::model::ScatterPlotWidgetDefinitionType,
    ) -> ScatterPlotWidgetDefinition {
        ScatterPlotWidgetDefinition {
            color_by_groups: None,
            custom_links: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            xaxis: None,
            yaxis: None,
        }
    }
}
