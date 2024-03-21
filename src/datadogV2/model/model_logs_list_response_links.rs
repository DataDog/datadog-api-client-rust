// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Links attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsListResponseLinks {
    /// Link for the next set of results. Note that the request can also be made using the
    /// POST endpoint.
    #[serde(rename = "next")]
    pub next: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsListResponseLinks {
    pub fn new() -> LogsListResponseLinks {
        LogsListResponseLinks {
            next: None,
            _unparsed: false,
        }
    }

    pub fn next(mut self, value: String) -> Self {
        self.next = Some(value);
        self
    }
}

impl Default for LogsListResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsListResponseLinks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsListResponseLinksVisitor;
        impl<'a> Visitor<'a> for LogsListResponseLinksVisitor {
            type Value = LogsListResponseLinks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next" => {
                            if v.is_null() {
                                continue;
                            }
                            next = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsListResponseLinks { next, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsListResponseLinksVisitor)
    }
}
