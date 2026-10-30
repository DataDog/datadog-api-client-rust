// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Static nodes and edges, stored in columns. Within a table, every column must have the same number of values and a unique `id`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyMapInlineData {
    /// Table of edges, one row per edge.
    #[serde(rename = "edges")]
    pub edges: Option<crate::datadogV1::model::TopologyMapInlineEdgesData>,
    /// Table of nodes, one row per node.
    #[serde(rename = "nodes")]
    pub nodes: Option<crate::datadogV1::model::TopologyMapInlineNodesData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyMapInlineData {
    pub fn new() -> TopologyMapInlineData {
        TopologyMapInlineData {
            edges: None,
            nodes: None,
            _unparsed: false,
        }
    }

    pub fn edges(mut self, value: crate::datadogV1::model::TopologyMapInlineEdgesData) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn nodes(mut self, value: crate::datadogV1::model::TopologyMapInlineNodesData) -> Self {
        self.nodes = Some(value);
        self
    }
}

impl Default for TopologyMapInlineData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TopologyMapInlineData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyMapInlineDataVisitor;
        impl<'a> Visitor<'a> for TopologyMapInlineDataVisitor {
            type Value = TopologyMapInlineData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut edges: Option<crate::datadogV1::model::TopologyMapInlineEdgesData> = None;
                let mut nodes: Option<crate::datadogV1::model::TopologyMapInlineNodesData> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "edges" => {
                            if v.is_null() {
                                continue;
                            }
                            edges = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nodes" => {
                            if v.is_null() {
                                continue;
                            }
                            nodes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TopologyMapInlineData {
                    edges,
                    nodes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyMapInlineDataVisitor)
    }
}
