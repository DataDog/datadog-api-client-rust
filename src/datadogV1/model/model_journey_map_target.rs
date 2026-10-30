// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The pair of journey steps the paths are visualized between.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JourneyMapTarget {
    /// Key of the end step in `search.node_objects`.
    #[serde(rename = "end")]
    pub end: String,
    /// Key of the start step in `search.node_objects`.
    #[serde(rename = "start")]
    pub start: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JourneyMapTarget {
    pub fn new(end: String, start: String) -> JourneyMapTarget {
        JourneyMapTarget {
            end,
            start,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for JourneyMapTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JourneyMapTargetVisitor;
        impl<'a> Visitor<'a> for JourneyMapTargetVisitor {
            type Value = JourneyMapTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<String> = None;
                let mut start: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = JourneyMapTarget {
                    end,
                    start,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JourneyMapTargetVisitor)
    }
}
