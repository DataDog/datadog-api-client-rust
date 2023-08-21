// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryMetricsSeriesMetadata {
    /// Query aggregator function.
    #[serde(rename = "aggr", skip_serializing_if = "Option::is_none")]
    pub aggr: String,
    /// Query expression.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: String,
    /// Query metric used.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Query index from original combined query.
    #[serde(rename = "query_index", skip_serializing_if = "Option::is_none")]
    pub query_index: i64,
    /// Query scope.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
    /// An array of metric units that contains up to two unit objects.
For example, bytes represents one unit object and bytes per second represents two unit objects.
If a metric query only has one unit object, the second array element is null.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub unit: Vec<SLOHistoryMetricsSeriesMetadataUnit>,
}

