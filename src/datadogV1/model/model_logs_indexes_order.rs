// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the ordered list of log index names.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsIndexesOrder {
    /// Array of strings identifying by their name(s) the index(es) of your organization.
    /// Logs are tested against the query filter of each index one by one, following the order of the array.
    /// Logs are eventually stored in the first matching index.
    #[serde(rename = "index_names")]
    pub index_names: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsIndexesOrder {
    pub fn new(index_names: Vec<String>) -> LogsIndexesOrder {
        LogsIndexesOrder {
            index_names,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for LogsIndexesOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsIndexesOrderVisitor;
        impl<'a> Visitor<'a> for LogsIndexesOrderVisitor {
            type Value = LogsIndexesOrder;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut index_names: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "index_names" => {
                            index_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let index_names =
                    index_names.ok_or_else(|| M::Error::missing_field("index_names"))?;

                let content = LogsIndexesOrder {
                    index_names,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsIndexesOrderVisitor)
    }
}
