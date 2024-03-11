// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A one-time downtime definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleOneTimeResponse {
    /// ISO-8601 Datetime to end the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<String>>,
    /// ISO-8601 Datetime to start the downtime.
    #[serde(rename = "start")]
    pub start: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleOneTimeResponse {
    pub fn new(start: String) -> DowntimeScheduleOneTimeResponse {
        DowntimeScheduleOneTimeResponse {
            end: None,
            start,
            _unparsed: false,
        }
    }

    pub fn end(&mut self, value: Option<String>) -> &mut Self {
        self.end = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleOneTimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleOneTimeResponseVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleOneTimeResponseVisitor {
            type Value = DowntimeScheduleOneTimeResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<Option<String>> = None;
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
                        &_ => {}
                    }
                }
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = DowntimeScheduleOneTimeResponse {
                    end,
                    start,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleOneTimeResponseVisitor)
    }
}
