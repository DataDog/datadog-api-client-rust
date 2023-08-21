// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionProcessQueryDefinition {
    /// The aggregation methods available for metrics queries.
    #[serde(rename = "aggregator", skip_serializing_if = "Option::is_none")]
    pub aggregator: FormulaAndFunctionMetricAggregation,
    /// Data sources that rely on the process backend.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: FormulaAndFunctionProcessQueryDataSource,
    /// Whether to normalize the CPU percentages.
    #[serde(rename = "is_normalized_cpu", skip_serializing_if = "Option::is_none")]
    pub is_normalized_cpu: bool,
    /// Number of hits to return.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// Process metric name.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Name of query for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Direction of sort.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: QuerySortOrder,
    /// An array of tags to filter by.
    #[serde(rename = "tag_filters", skip_serializing_if = "Option::is_none")]
    pub tag_filters: Vec<String>,
    /// Text to use as filter.
    #[serde(rename = "text_filter", skip_serializing_if = "Option::is_none")]
    pub text_filter: String,
}

