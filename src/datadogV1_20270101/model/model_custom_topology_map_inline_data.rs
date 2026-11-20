// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomTopologyMapInlineData {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "edges")]
    pub edges: Option<crate::datadogV1_20270101::model::CustomTopologyMapInlineEdgesData>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "nodes")]
    pub nodes: Option<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodesData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CustomTopologyMapInlineData {
    pub fn new() -> CustomTopologyMapInlineData {
        CustomTopologyMapInlineData {
            edges: None,
            nodes: None,
            _unparsed: false,
        }
    }
    
    pub fn edges(mut self, value: crate::datadogV1_20270101::model::CustomTopologyMapInlineEdgesData) -> Self {
        self.edges = Some(value);
        self
    }
    
    pub fn nodes(mut self, value: crate::datadogV1_20270101::model::CustomTopologyMapInlineNodesData) -> Self {
        self.nodes = Some(value);
        self
    }
    
}


impl Default for CustomTopologyMapInlineData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomTopologyMapInlineData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomTopologyMapInlineDataVisitor;
        impl<'a> Visitor<'a> for CustomTopologyMapInlineDataVisitor {
            type Value = CustomTopologyMapInlineData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut edges: Option<crate::datadogV1_20270101::model::CustomTopologyMapInlineEdgesData> = None;
                let mut nodes: Option<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodesData> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "edges" => {
                            if v.is_null() {
                                continue;
                            }
                            edges = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "nodes" => {
                            if v.is_null() {
                                continue;
                            }
                            nodes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = CustomTopologyMapInlineData {
                    edges,
                    nodes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomTopologyMapInlineDataVisitor)
    }
}
