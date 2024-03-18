// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing how to split the graph to display multiple visualizations per request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookSplitBy {
    /// Keys to split on.
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
    /// Tags to split on.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookSplitBy {
    pub fn new(keys: Vec<String>, tags: Vec<String>) -> NotebookSplitBy {
        NotebookSplitBy {
            keys,
            tags,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookSplitBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookSplitByVisitor;
        impl<'a> Visitor<'a> for NotebookSplitByVisitor {
            type Value = NotebookSplitBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut keys: Option<Vec<String>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "keys" => {
                            keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let keys = keys.ok_or_else(|| M::Error::missing_field("keys"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = NotebookSplitBy {
                    keys,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookSplitByVisitor)
    }
}
