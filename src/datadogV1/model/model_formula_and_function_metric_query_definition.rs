// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A formula and functions metrics query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionMetricQueryDefinition {
    /// The aggregation methods available for metrics queries.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::FormulaAndFunctionMetricAggregation>,
    /// Data source for metrics queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionMetricDataSource,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Metrics query definition.
    #[serde(rename = "query")]
    pub query: String,
}

impl FormulaAndFunctionMetricQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionMetricDataSource,
        name: String,
        query: String,
    ) -> FormulaAndFunctionMetricQueryDefinition {
        FormulaAndFunctionMetricQueryDefinition {
            aggregator: None,
            data_source,
            name,
            query,
        }
    }
}
