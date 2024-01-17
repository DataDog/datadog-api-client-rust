// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated change widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Show the absolute or the relative change.
    #[serde(rename = "change_type")]
    pub change_type: Option<crate::datadogV1::model::WidgetChangeType>,
    /// Timeframe used for the change comparison.
    #[serde(rename = "compare_to")]
    pub compare_to: Option<crate::datadogV1::model::WidgetCompareTo>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// Whether to show increase as good.
    #[serde(rename = "increase_good")]
    pub increase_good: Option<bool>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// What to order by.
    #[serde(rename = "order_by")]
    pub order_by: Option<crate::datadogV1::model::WidgetOrderBy>,
    /// Widget sorting methods.
    #[serde(rename = "order_dir")]
    pub order_dir: Option<crate::datadogV1::model::WidgetSort>,
    /// The process query to use in the widget.
    #[serde(rename = "process_query")]
    pub process_query: Option<Box<crate::datadogV1::model::ProcessQueryDefinition>>,
    /// The log query.
    #[serde(rename = "profile_metrics_query")]
    pub profile_metrics_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Query definition.
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
    /// Whether to show the present value.
    #[serde(rename = "show_present")]
    pub show_present: Option<bool>,
}

impl ChangeWidgetRequest {
    pub fn new() -> ChangeWidgetRequest {
        ChangeWidgetRequest {
            apm_query: None,
            change_type: None,
            compare_to: None,
            event_query: None,
            formulas: None,
            increase_good: None,
            log_query: None,
            network_query: None,
            order_by: None,
            order_dir: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            show_present: None,
        }
    }
}
