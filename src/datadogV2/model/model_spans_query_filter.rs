// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The search and filter query settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansQueryFilter {
    /// The minimum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// The search query - following the span search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The maximum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "to")]
    pub to: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansQueryFilter {
    pub fn new() -> SpansQueryFilter {
        SpansQueryFilter {
            from: None,
            query: None,
            to: None,
            _unparsed: false,
        }
    }

    pub fn from(mut self, value: String) -> Self {
        self.from = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn to(mut self, value: String) -> Self {
        self.to = Some(value);
        self
    }
}

impl Default for SpansQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansQueryFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansQueryFilterVisitor;
        impl<'a> Visitor<'a> for SpansQueryFilterVisitor {
            type Value = SpansQueryFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<String> = None;
                let mut query: Option<String> = None;
                let mut to: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SpansQueryFilter {
                    from,
                    query,
                    to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansQueryFilterVisitor)
    }
}
