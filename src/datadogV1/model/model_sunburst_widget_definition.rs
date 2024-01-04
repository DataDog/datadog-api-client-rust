// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Sunbursts are spot on to highlight how groups contribute to the total of a query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SunburstWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Show the total value in this widget.
    #[serde(rename = "hide_total")]
    pub hide_total: Option<bool>,
    /// Configuration of the legend.
    #[serde(rename = "legend")]
    pub legend: Option<Box<crate::datadogV1::model::SunburstWidgetLegend>>,
    /// List of sunburst widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::SunburstWidgetRequest>,
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
    /// Type of the Sunburst widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SunburstWidgetDefinitionType,
}

impl SunburstWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::SunburstWidgetRequest>,
        type_: crate::datadogV1::model::SunburstWidgetDefinitionType,
    ) -> SunburstWidgetDefinition {
        SunburstWidgetDefinition {
            custom_links: None,
            hide_total: None,
            legend: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}