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
pub struct GraphQuerySearch {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filters")]
    pub filters: Option<crate::datadogV1_20270101::model::GraphQuerySearchFilters>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "join_keys")]
    pub join_keys: Option<crate::datadogV1_20270101::model::GraphQuerySearchJoinKeys>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "nodes")]
    pub nodes: std::collections::BTreeMap<String, String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GraphQuerySearch {
    pub fn new(expression: String, nodes: std::collections::BTreeMap<String, String>) -> GraphQuerySearch {
        GraphQuerySearch {
            expression,
            filters: None,
            join_keys: None,
            nodes,
            _unparsed: false,
        }
    }
    
    pub fn filters(mut self, value: crate::datadogV1_20270101::model::GraphQuerySearchFilters) -> Self {
        self.filters = Some(value);
        self
    }
    
    pub fn join_keys(mut self, value: crate::datadogV1_20270101::model::GraphQuerySearchJoinKeys) -> Self {
        self.join_keys = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GraphQuerySearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GraphQuerySearchVisitor;
        impl<'a> Visitor<'a> for GraphQuerySearchVisitor {
            type Value = GraphQuerySearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expression: Option<String> = None;
                let mut filters: Option<crate::datadogV1_20270101::model::GraphQuerySearchFilters> = None;
                let mut join_keys: Option<crate::datadogV1_20270101::model::GraphQuerySearchJoinKeys> = None;
                let mut nodes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "join_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            join_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "nodes" => {
                            nodes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let nodes = nodes.ok_or_else(|| M::Error::missing_field("nodes"))?;

                let content = GraphQuerySearch {
                    expression,
                    filters,
                    join_keys,
                    nodes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GraphQuerySearchVisitor)
    }
}
