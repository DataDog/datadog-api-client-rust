// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Query metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetricsSeriesMetadata {
    /// Query aggregator function.
    #[serde(rename = "aggr")]
    pub aggr: Option<String>,
    /// Query expression.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// Query metric used.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Query index from original combined query.
    #[serde(rename = "query_index")]
    pub query_index: Option<i64>,
    /// Query scope.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// An array of metric units that contains up to two unit objects.
    /// For example, bytes represents one unit object and bytes per second represents two unit objects.
    /// If a metric query only has one unit object, the second array element is null.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit:
        Option<Option<Vec<Option<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadataUnit>>>>,
}

impl SLOHistoryMetricsSeriesMetadata {
    pub fn new() -> SLOHistoryMetricsSeriesMetadata {
        SLOHistoryMetricsSeriesMetadata {
            aggr: None,
            expression: None,
            metric: None,
            query_index: None,
            scope: None,
            unit: None,
        }
    }
}
