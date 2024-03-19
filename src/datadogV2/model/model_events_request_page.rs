// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsRequestPage {
    /// The returned paging point to use to get the next results.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// The maximum number of logs in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsRequestPage {
    pub fn new() -> EventsRequestPage {
        EventsRequestPage {
            cursor: None,
            limit: None,
            _unparsed: false,
        }
    }

    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }

    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
}

impl Default for EventsRequestPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventsRequestPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsRequestPageVisitor;
        impl<'a> Visitor<'a> for EventsRequestPageVisitor {
            type Value = EventsRequestPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cursor: Option<String> = None;
                let mut limit: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            cursor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = EventsRequestPage {
                    cursor,
                    limit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsRequestPageVisitor)
    }
}
