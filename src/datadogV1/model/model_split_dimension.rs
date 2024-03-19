// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The property by which the graph splits
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SplitDimension {
    /// The system interprets this attribute differently depending on the data source of the query being split. For metrics, it's a tag. For the events platform, it's an attribute or tag.
    #[serde(rename = "one_graph_per")]
    pub one_graph_per: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SplitDimension {
    pub fn new(one_graph_per: String) -> SplitDimension {
        SplitDimension {
            one_graph_per,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SplitDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitDimensionVisitor;
        impl<'a> Visitor<'a> for SplitDimensionVisitor {
            type Value = SplitDimension;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut one_graph_per: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "one_graph_per" => {
                            one_graph_per =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let one_graph_per =
                    one_graph_per.ok_or_else(|| M::Error::missing_field("one_graph_per"))?;

                let content = SplitDimension {
                    one_graph_per,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitDimensionVisitor)
    }
}
