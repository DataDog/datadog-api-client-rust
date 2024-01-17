// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The heat map visualization shows metrics aggregated across many tags, such as hosts. The more hosts that have a particular value, the darker that square is.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// List of widget events.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV1::model::WidgetEvent>>,
    /// Available legend sizes for a widget. Should be one of "0", "2", "4", "8", "16", or "auto".
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// List of widget types.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::HeatMapWidgetRequest>,
    /// Whether or not to display the legend on this widget.
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<Box<crate::datadogV1::model::WidgetTime>>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the heat map widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::HeatMapWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<Box<crate::datadogV1::model::WidgetAxis>>,
}

impl HeatMapWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::HeatMapWidgetRequest>,
        type_: crate::datadogV1::model::HeatMapWidgetDefinitionType,
    ) -> HeatMapWidgetDefinition {
        HeatMapWidgetDefinition {
            custom_links: None,
            events: None,
            legend_size: None,
            requests,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            yaxis: None,
        }
    }
}
