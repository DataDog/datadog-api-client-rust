// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use the SLO and uptime widget to track your SLOs (Service Level Objectives) and uptime on screenboards and timeboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOWidgetDefinition {
    /// Additional filters applied to the SLO query.
    #[serde(rename = "additional_query_filters")]
    pub additional_query_filters: Option<String>,
    /// Defined global time target.
    #[serde(rename = "global_time_target")]
    pub global_time_target: Option<String>,
    /// Defined error budget.
    #[serde(rename = "show_error_budget")]
    pub show_error_budget: Option<bool>,
    /// ID of the SLO displayed.
    #[serde(rename = "slo_id")]
    pub slo_id: Option<String>,
    /// Times being monitored.
    #[serde(rename = "time_windows")]
    pub time_windows: Option<Vec<crate::datadogV1::model::WidgetTimeWindows>>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the SLO widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SLOWidgetDefinitionType,
    /// Define how you want the SLO to be displayed.
    #[serde(rename = "view_mode")]
    pub view_mode: Option<crate::datadogV1::model::WidgetViewMode>,
    /// Type of view displayed by the widget.
    #[serde(rename = "view_type")]
    pub view_type: String,
}

impl SLOWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::SLOWidgetDefinitionType,
        view_type: String,
    ) -> SLOWidgetDefinition {
        SLOWidgetDefinition {
            additional_query_filters: None,
            global_time_target: None,
            show_error_budget: None,
            slo_id: None,
            time_windows: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            view_mode: None,
            view_type,
        }
    }
}
