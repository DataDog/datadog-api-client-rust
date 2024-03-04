// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A `metric` based SLO history response.
///
/// This is not included in responses for `monitor` based SLOs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetrics {
    /// A representation of `metric` based SLO time series for the provided queries.
    /// This is the same response type from `batch_query` endpoint.
    #[serde(rename = "denominator")]
    pub denominator: crate::datadogV1::model::SLOHistoryMetricsSeries,
    /// The aggregated query interval for the series data. It's implicit based on the query time window.
    #[serde(rename = "interval")]
    pub interval: i64,
    /// Optional message if there are specific query issues/warnings.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// A representation of `metric` based SLO time series for the provided queries.
    /// This is the same response type from `batch_query` endpoint.
    #[serde(rename = "numerator")]
    pub numerator: crate::datadogV1::model::SLOHistoryMetricsSeries,
    /// The combined numerator and denominator query CSV.
    #[serde(rename = "query")]
    pub query: String,
    /// The series result type. This mimics `batch_query` response type.
    #[serde(rename = "res_type")]
    pub res_type: String,
    /// The series response version type. This mimics `batch_query` response type.
    #[serde(rename = "resp_version")]
    pub resp_version: i64,
    /// An array of query timestamps in EPOCH milliseconds.
    #[serde(rename = "times")]
    pub times: Vec<f64>,
}

impl SLOHistoryMetrics {
    pub fn new(
        denominator: crate::datadogV1::model::SLOHistoryMetricsSeries,
        interval: i64,
        numerator: crate::datadogV1::model::SLOHistoryMetricsSeries,
        query: String,
        res_type: String,
        resp_version: i64,
        times: Vec<f64>,
    ) -> SLOHistoryMetrics {
        SLOHistoryMetrics {
            denominator,
            interval,
            message: None,
            numerator,
            query,
            res_type,
            resp_version,
            times,
        }
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}
