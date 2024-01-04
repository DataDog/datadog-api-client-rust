// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Query values display the current value of a given metric, APM, or log query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryValueWidgetDefinition {
    /// Whether to use auto-scaling or not.
    #[serde(rename = "autoscale")]
    pub autoscale: Option<bool>,
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Display a unit of your choice on the widget.
    #[serde(rename = "custom_unit")]
    pub custom_unit: Option<String>,
    /// Number of decimals to show. If not defined, the widget uses the raw value.
    #[serde(rename = "precision")]
    pub precision: Option<i64>,
    /// Widget definition.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::QueryValueWidgetRequest>,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<Box<crate::datadogV1::model::WidgetTime>>,
    /// Set a timeseries on the widget background.
    #[serde(rename = "timeseries_background")]
    pub timeseries_background: Option<Box<crate::datadogV1::model::TimeseriesBackground>>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the query value widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::QueryValueWidgetDefinitionType,
}

impl QueryValueWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::QueryValueWidgetRequest>,
        type_: crate::datadogV1::model::QueryValueWidgetDefinitionType,
    ) -> QueryValueWidgetDefinition {
        QueryValueWidgetDefinition {
            autoscale: None,
            custom_links: None,
            custom_unit: None,
            precision: None,
            requests,
            text_align: None,
            time: None,
            timeseries_background: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}