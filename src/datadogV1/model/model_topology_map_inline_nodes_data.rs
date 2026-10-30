// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Table of nodes, one row per node.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyMapInlineNodesData {
    /// Columns of the nodes table. Must include an `id` column, unless the list is empty (no nodes).
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1::model::TopologyMapInlineNodeColumn>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyMapInlineNodesData {
    pub fn new(
        columns: Vec<crate::datadogV1::model::TopologyMapInlineNodeColumn>,
    ) -> TopologyMapInlineNodesData {
        TopologyMapInlineNodesData {
            columns,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for TopologyMapInlineNodesData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyMapInlineNodesDataVisitor;
        impl<'a> Visitor<'a> for TopologyMapInlineNodesDataVisitor {
            type Value = TopologyMapInlineNodesData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1::model::TopologyMapInlineNodeColumn>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;

                let content = TopologyMapInlineNodesData { columns, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyMapInlineNodesDataVisitor)
    }
}
