// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Process query using formulas and functions.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionProcessQueryDefinition {
    /// The aggregation methods available for metrics queries.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::FormulaAndFunctionMetricAggregation>,
    /// Data sources that rely on the process backend.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionProcessQueryDataSource,
    /// Whether to normalize the CPU percentages.
    #[serde(rename = "is_normalized_cpu")]
    pub is_normalized_cpu: Option<bool>,
    /// Number of hits to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Process metric name.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Name of query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Direction of sort.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::QuerySortOrder>,
    /// An array of tags to filter by.
    #[serde(rename = "tag_filters")]
    pub tag_filters: Option<Vec<String>>,
    /// Text to use as filter.
    #[serde(rename = "text_filter")]
    pub text_filter: Option<String>,
}

impl FormulaAndFunctionProcessQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionProcessQueryDataSource,
        metric: String,
        name: String,
    ) -> FormulaAndFunctionProcessQueryDefinition {
        FormulaAndFunctionProcessQueryDefinition {
            aggregator: None,
            data_source,
            is_normalized_cpu: None,
            limit: None,
            metric,
            name,
            sort: None,
            tag_filters: None,
            text_filter: None,
        }
    }
}