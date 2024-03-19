// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The event query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventQueryDefinition {
    /// The query being made on the event.
    #[serde(rename = "search")]
    pub search: String,
    /// The execution method for multi-value filters. Can be either and or or.
    #[serde(rename = "tags_execution")]
    pub tags_execution: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventQueryDefinition {
    pub fn new(search: String, tags_execution: String) -> EventQueryDefinition {
        EventQueryDefinition {
            search,
            tags_execution,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for EventQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for EventQueryDefinitionVisitor {
            type Value = EventQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut search: Option<String> = None;
                let mut tags_execution: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags_execution" => {
                            tags_execution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;
                let tags_execution =
                    tags_execution.ok_or_else(|| M::Error::missing_field("tags_execution"))?;

                let content = EventQueryDefinition {
                    search,
                    tags_execution,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventQueryDefinitionVisitor)
    }
}
