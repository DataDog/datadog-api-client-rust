// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetricsSeries {
    /// Count of submitted metrics.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i64,
    /// Query metadata.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: SLOHistoryMetricsSeriesMetadata,
    /// Total sum of the query.
    #[serde(rename = "sum", skip_serializing_if = "Option::is_none")]
    pub sum: f64,
    /// The query values for each metric.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Vec<f64>,
}

