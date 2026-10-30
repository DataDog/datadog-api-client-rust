// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A path expanded to show the events that follow it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JourneyMapExpandedPrefix {
    /// Events of the expanded path, in order.
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// Maximum number of paths shown after the expanded path.
    #[serde(rename = "path_limit")]
    pub path_limit: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JourneyMapExpandedPrefix {
    pub fn new(events: Vec<String>) -> JourneyMapExpandedPrefix {
        JourneyMapExpandedPrefix {
            events,
            path_limit: None,
            _unparsed: false,
        }
    }

    pub fn path_limit(mut self, value: i64) -> Self {
        self.path_limit = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for JourneyMapExpandedPrefix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JourneyMapExpandedPrefixVisitor;
        impl<'a> Visitor<'a> for JourneyMapExpandedPrefixVisitor {
            type Value = JourneyMapExpandedPrefix;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut events: Option<Vec<String>> = None;
                let mut path_limit: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "events" => {
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            path_limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let events = events.ok_or_else(|| M::Error::missing_field("events"))?;

                let content = JourneyMapExpandedPrefix {
                    events,
                    path_limit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JourneyMapExpandedPrefixVisitor)
    }
}
