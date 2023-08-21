// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphSnapshot {
    /// A JSON document defining the graph. `graph_def` can be used instead of `metric_query`.
The JSON document uses the [grammar defined here](https://docs.datadoghq.com/graphing/graphing_json/#grammar)
and should be formatted to a single line then URL encoded.
    #[serde(rename = "graph_def", skip_serializing_if = "Option::is_none")]
    pub graph_def: String,
    /// The metric query. One of `metric_query` or `graph_def` is required.
    #[serde(rename = "metric_query", skip_serializing_if = "Option::is_none")]
    pub metric_query: String,
    /// URL of your [graph snapshot](https://docs.datadoghq.com/metrics/explorer/#snapshot).
    #[serde(rename = "snapshot_url", skip_serializing_if = "Option::is_none")]
    pub snapshot_url: String,
}

