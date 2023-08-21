// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterplotTableRequest {
    /// List of Scatterplot formulas that operate on queries.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Vec<ScatterplotWidgetFormula>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<FormulaAndFunctionQueryDefinition>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: FormulaAndFunctionResponseFormat,
}

