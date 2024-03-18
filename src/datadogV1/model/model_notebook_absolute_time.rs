// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Absolute timeframe.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookAbsoluteTime {
    /// The end time.
    #[serde(rename = "end")]
    pub end: DateTime<Utc>,
    /// Indicates whether the timeframe should be shifted to end at the current time.
    #[serde(rename = "live")]
    pub live: Option<bool>,
    /// The start time.
    #[serde(rename = "start")]
    pub start: DateTime<Utc>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookAbsoluteTime {
    pub fn new(end: DateTime<Utc>, start: DateTime<Utc>) -> NotebookAbsoluteTime {
        NotebookAbsoluteTime {
            end,
            live: None,
            start,
            _unparsed: false,
        }
    }

    pub fn live(mut self, value: bool) -> Self {
        self.live = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NotebookAbsoluteTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookAbsoluteTimeVisitor;
        impl<'a> Visitor<'a> for NotebookAbsoluteTimeVisitor {
            type Value = NotebookAbsoluteTime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<DateTime<Utc>> = None;
                let mut live: Option<bool> = None;
                let mut start: Option<DateTime<Utc>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "live" => {
                            if v.is_null() {
                                continue;
                            }
                            live = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = NotebookAbsoluteTime {
                    end,
                    live,
                    start,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookAbsoluteTimeVisitor)
    }
}
