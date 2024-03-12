// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object representing a graph snapshot.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphSnapshot {
    /// A JSON document defining the graph. `graph_def` can be used instead of `metric_query`.
    /// The JSON document uses the [grammar defined here](<https://docs.datadoghq.com/graphing/graphing_json/#grammar>)
    /// and should be formatted to a single line then URL encoded.
    #[serde(rename = "graph_def")]
    pub graph_def: Option<String>,
    /// The metric query. One of `metric_query` or `graph_def` is required.
    #[serde(rename = "metric_query")]
    pub metric_query: Option<String>,
    /// URL of your [graph snapshot](<https://docs.datadoghq.com/metrics/explorer/#snapshot>).
    #[serde(rename = "snapshot_url")]
    pub snapshot_url: Option<String>,
}

impl GraphSnapshot {
    pub fn new() -> GraphSnapshot {
        GraphSnapshot {
            graph_def: None,
            metric_query: None,
            snapshot_url: None,
        }
    }

    pub fn graph_def(mut self, value: String) -> Self {
        self.graph_def = Some(value);
        self
    }

    pub fn metric_query(mut self, value: String) -> Self {
        self.metric_query = Some(value);
        self
    }

    pub fn snapshot_url(mut self, value: String) -> Self {
        self.snapshot_url = Some(value);
        self
    }
}

impl Default for GraphSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
