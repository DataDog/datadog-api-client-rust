// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing a graph snapshot.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GraphSnapshot {
    pub fn new() -> GraphSnapshot {
        GraphSnapshot {
            graph_def: None,
            metric_query: None,
            snapshot_url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for GraphSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GraphSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GraphSnapshotVisitor;
        impl<'a> Visitor<'a> for GraphSnapshotVisitor {
            type Value = GraphSnapshot;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut graph_def: Option<String> = None;
                let mut metric_query: Option<String> = None;
                let mut snapshot_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "graph_def" => {
                            if v.is_null() {
                                continue;
                            }
                            graph_def = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_query" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot_url" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GraphSnapshot {
                    graph_def,
                    metric_query,
                    snapshot_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GraphSnapshotVisitor)
    }
}
