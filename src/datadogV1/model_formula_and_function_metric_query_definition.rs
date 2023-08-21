// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionMetricQueryDefinition {
    /// The aggregation methods available for metrics queries.
    #[serde(rename = "aggregator", skip_serializing_if = "Option::is_none")]
    pub aggregator: FormulaAndFunctionMetricAggregation,
    /// Data source for metrics queries.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: FormulaAndFunctionMetricDataSource,
    /// Name of the query for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Metrics query definition.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
}

