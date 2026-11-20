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
pub struct CustomTopologyMapInlineEdgesData {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1_20270101::model::CustomTopologyMapInlineEdgeColumn>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CustomTopologyMapInlineEdgesData {
    pub fn new(columns: Vec<crate::datadogV1_20270101::model::CustomTopologyMapInlineEdgeColumn>) -> CustomTopologyMapInlineEdgesData {
        CustomTopologyMapInlineEdgesData {
            columns,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for CustomTopologyMapInlineEdgesData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomTopologyMapInlineEdgesDataVisitor;
        impl<'a> Visitor<'a> for CustomTopologyMapInlineEdgesDataVisitor {
            type Value = CustomTopologyMapInlineEdgesData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::CustomTopologyMapInlineEdgeColumn>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;

                let content = CustomTopologyMapInlineEdgesData {
                    columns,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomTopologyMapInlineEdgesDataVisitor)
    }
}
