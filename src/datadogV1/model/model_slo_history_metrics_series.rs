// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A representation of `metric` based SLO time series for the provided queries.
/// This is the same response type from `batch_query` endpoint.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetricsSeries {
    /// Count of submitted metrics.
    #[serde(rename = "count")]
    pub count: i64,
    /// Query metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<Box<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadata>>,
    /// Total sum of the query.
    #[serde(rename = "sum")]
    pub sum: f64,
    /// The query values for each metric.
    #[serde(rename = "values")]
    pub values: Vec<f64>,
}

impl SLOHistoryMetricsSeries {
    pub fn new(count: i64, sum: f64, values: Vec<f64>) -> SLOHistoryMetricsSeries {
        SLOHistoryMetricsSeries {
            count,
            metadata: None,
            sum,
            values,
        }
    }
}
