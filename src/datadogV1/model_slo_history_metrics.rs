// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetrics {
    /// A representation of `metric` based SLO time series for the provided queries.
This is the same response type from `batch_query` endpoint.
    #[serde(rename = "denominator")]
    pub denominator: SLOHistoryMetricsSeries,
    /// The aggregated query interval for the series data. It's implicit based on the query time window.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
    /// Optional message if there are specific query issues/warnings.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// A representation of `metric` based SLO time series for the provided queries.
This is the same response type from `batch_query` endpoint.
    #[serde(rename = "numerator")]
    pub numerator: SLOHistoryMetricsSeries,
    /// The combined numerator and denominator query CSV.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The series result type. This mimics `batch_query` response type.
    #[serde(rename = "res_type", skip_serializing_if = "Option::is_none")]
    pub res_type: String,
    /// The series response version type. This mimics `batch_query` response type.
    #[serde(rename = "resp_version", skip_serializing_if = "Option::is_none")]
    pub resp_version: i64,
    /// An array of query timestamps in EPOCH milliseconds.
    #[serde(rename = "times", skip_serializing_if = "Option::is_none")]
    pub times: Vec<f64>,
}

