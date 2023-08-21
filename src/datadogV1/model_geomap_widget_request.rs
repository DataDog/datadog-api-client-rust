// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetRequest {
    /// Widget columns.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Vec<ListStreamColumn>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Vec<WidgetFormula>,
    /// The log query.
    #[serde(rename = "log_query", skip_serializing_if = "Option::is_none")]
    pub log_query: LogQueryDefinition,
    /// The widget metrics query.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: String,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<FormulaAndFunctionQueryDefinition>,
    /// Updated list stream widget.
    #[serde(rename = "query")]
    pub query: ListStreamQuery,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: FormulaAndFunctionResponseFormat,
    /// The log query.
    #[serde(rename = "rum_query", skip_serializing_if = "Option::is_none")]
    pub rum_query: LogQueryDefinition,
    /// The log query.
    #[serde(rename = "security_query", skip_serializing_if = "Option::is_none")]
    pub security_query: LogQueryDefinition,
}

