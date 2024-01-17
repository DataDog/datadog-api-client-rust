// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated timeseries widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The log query.
    #[serde(rename = "audit_query")]
    pub audit_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Type of display to use for the request.
    #[serde(rename = "display_type")]
    pub display_type: Option<crate::datadogV1::model::WidgetDisplayType>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Used to define expression aliases.
    #[serde(rename = "metadata")]
    pub metadata: Option<Vec<crate::datadogV1::model::TimeseriesWidgetExpressionAlias>>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Whether or not to display a second y-axis on the right.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
    /// The process query to use in the widget.
    #[serde(rename = "process_query")]
    pub process_query: Option<Box<crate::datadogV1::model::ProcessQueryDefinition>>,
    /// The log query.
    #[serde(rename = "profile_metrics_query")]
    pub profile_metrics_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Widget query.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::datadogV1::model::FormulaAndFunctionResponseFormat>,
    /// The log query.
    #[serde(rename = "rum_query")]
    pub rum_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The log query.
    #[serde(rename = "security_query")]
    pub security_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Define request widget style.
    #[serde(rename = "style")]
    pub style: Option<Box<crate::datadogV1::model::WidgetRequestStyle>>,
}

impl TimeseriesWidgetRequest {
    pub fn new() -> TimeseriesWidgetRequest {
        TimeseriesWidgetRequest {
            apm_query: None,
            audit_query: None,
            display_type: None,
            event_query: None,
            formulas: None,
            log_query: None,
            metadata: None,
            network_query: None,
            on_right_yaxis: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            style: None,
        }
    }
}
impl Default for TimeseriesWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}
