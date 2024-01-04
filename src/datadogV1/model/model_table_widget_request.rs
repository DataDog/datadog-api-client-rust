// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated table widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableWidgetRequest {
    /// Aggregator used for the request.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::WidgetAggregator>,
    /// The column name (defaults to the metric name).
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The APM stats query for table and distributions widgets.
    #[serde(rename = "apm_stats_query")]
    pub apm_stats_query: Option<Box<crate::datadogV1::model::ApmStatsQueryDefinition>>,
    /// A list of display modes for each table cell.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<Vec<crate::datadogV1::model::TableWidgetCellDisplayMode>>,
    /// List of conditional formats.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// For metric queries, the number of lines to show in the table. Only one request should have this property.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::WidgetSort>,
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
}

impl TableWidgetRequest {
    pub fn new() -> TableWidgetRequest {
        TableWidgetRequest {
            aggregator: None,
            alias: None,
            apm_query: None,
            apm_stats_query: None,
            cell_display_mode: None,
            conditional_formats: None,
            event_query: None,
            formulas: None,
            limit: None,
            log_query: None,
            network_query: None,
            order: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
        }
    }
}
