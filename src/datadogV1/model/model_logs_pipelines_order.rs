// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the ordered list of pipeline IDs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsPipelinesOrder {
    /// Ordered Array of `<PIPELINE_ID>` strings, the order of pipeline IDs in the array
    /// define the overall Pipelines order for Datadog.
    #[serde(rename = "pipeline_ids")]
    pub pipeline_ids: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsPipelinesOrder {
    pub fn new(pipeline_ids: Vec<String>) -> LogsPipelinesOrder {
        LogsPipelinesOrder {
            pipeline_ids,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for LogsPipelinesOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsPipelinesOrderVisitor;
        impl<'a> Visitor<'a> for LogsPipelinesOrderVisitor {
            type Value = LogsPipelinesOrder;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pipeline_ids: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pipeline_ids" => {
                            pipeline_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let pipeline_ids =
                    pipeline_ids.ok_or_else(|| M::Error::missing_field("pipeline_ids"))?;

                let content = LogsPipelinesOrder {
                    pipeline_ids,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsPipelinesOrderVisitor)
    }
}
