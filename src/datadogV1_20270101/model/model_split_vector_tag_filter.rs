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
pub struct SplitVectorTagFilter {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tag_values")]
    pub tag_values: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SplitVectorTagFilter {
    pub fn new(tag_key: String, tag_values: Vec<String>) -> SplitVectorTagFilter {
        SplitVectorTagFilter {
            tag_key,
            tag_values,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for SplitVectorTagFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitVectorTagFilterVisitor;
        impl<'a> Visitor<'a> for SplitVectorTagFilterVisitor {
            type Value = SplitVectorTagFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tag_key: Option<String> = None;
                let mut tag_values: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "tag_values" => {
                            tag_values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_values = tag_values.ok_or_else(|| M::Error::missing_field("tag_values"))?;

                let content = SplitVectorTagFilter {
                    tag_key,
                    tag_values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitVectorTagFilterVisitor)
    }
}
