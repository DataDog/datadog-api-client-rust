// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query", skip_serializing_if = "Option::is_none")]
    pub apm_query: LogQueryDefinition,
    /// Show the absolute or the relative change.
    #[serde(rename = "change_type", skip_serializing_if = "Option::is_none")]
    pub change_type: WidgetChangeType,
    /// Timeframe used for the change comparison.
    #[serde(rename = "compare_to", skip_serializing_if = "Option::is_none")]
    pub compare_to: WidgetCompareTo,
    /// The log query.
    #[serde(rename = "event_query", skip_serializing_if = "Option::is_none")]
    pub event_query: LogQueryDefinition,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Vec<WidgetFormula>,
    /// Whether to show increase as good.
    #[serde(rename = "increase_good", skip_serializing_if = "Option::is_none")]
    pub increase_good: bool,
    /// The log query.
    #[serde(rename = "log_query", skip_serializing_if = "Option::is_none")]
    pub log_query: LogQueryDefinition,
    /// The log query.
    #[serde(rename = "network_query", skip_serializing_if = "Option::is_none")]
    pub network_query: LogQueryDefinition,
    /// What to order by.
    #[serde(rename = "order_by", skip_serializing_if = "Option::is_none")]
    pub order_by: WidgetOrderBy,
    /// Widget sorting methods.
    #[serde(rename = "order_dir", skip_serializing_if = "Option::is_none")]
    pub order_dir: WidgetSort,
    /// The process query to use in the widget.
    #[serde(rename = "process_query")]
    pub process_query: ProcessQueryDefinition,
    /// The log query.
    #[serde(rename = "profile_metrics_query", skip_serializing_if = "Option::is_none")]
    pub profile_metrics_query: LogQueryDefinition,
    /// Query definition.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: String,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<FormulaAndFunctionQueryDefinition>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: FormulaAndFunctionResponseFormat,
    /// The log query.
    #[serde(rename = "rum_query", skip_serializing_if = "Option::is_none")]
    pub rum_query: LogQueryDefinition,
    /// The log query.
    #[serde(rename = "security_query", skip_serializing_if = "Option::is_none")]
    pub security_query: LogQueryDefinition,
    /// Whether to show the present value.
    #[serde(rename = "show_present", skip_serializing_if = "Option::is_none")]
    pub show_present: bool,
}

