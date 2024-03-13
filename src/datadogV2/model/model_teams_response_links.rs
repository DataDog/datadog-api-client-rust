// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Teams response links.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsResponseLinks {
    /// First link.
    #[serde(rename = "first")]
    pub first: Option<String>,
    /// Last link.
    #[serde(rename = "last", default, with = "::serde_with::rust::double_option")]
    pub last: Option<Option<String>>,
    /// Next link.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Previous link.
    #[serde(rename = "prev", default, with = "::serde_with::rust::double_option")]
    pub prev: Option<Option<String>>,
    /// Current link.
    #[serde(rename = "self")]
    pub self_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsResponseLinks {
    pub fn new() -> TeamsResponseLinks {
        TeamsResponseLinks {
            first: None,
            last: None,
            next: None,
            prev: None,
            self_: None,
            _unparsed: false,
        }
    }

    pub fn first(mut self, value: String) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: Option<String>) -> Self {
        self.last = Some(value);
        self
    }

    pub fn next(mut self, value: String) -> Self {
        self.next = Some(value);
        self
    }

    pub fn prev(mut self, value: Option<String>) -> Self {
        self.prev = Some(value);
        self
    }

    pub fn self_(mut self, value: String) -> Self {
        self.self_ = Some(value);
        self
    }
}

impl Default for TeamsResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamsResponseLinks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsResponseLinksVisitor;
        impl<'a> Visitor<'a> for TeamsResponseLinksVisitor {
            type Value = TeamsResponseLinks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first: Option<String> = None;
                let mut last: Option<Option<String>> = None;
                let mut next: Option<String> = None;
                let mut prev: Option<Option<String>> = None;
                let mut self_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first" => {
                            if v.is_null() {
                                continue;
                            }
                            first = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last" => {
                            last = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next" => {
                            if v.is_null() {
                                continue;
                            }
                            next = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev" => {
                            prev = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "self" => {
                            if v.is_null() {
                                continue;
                            }
                            self_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = TeamsResponseLinks {
                    first,
                    last,
                    next,
                    prev,
                    self_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsResponseLinksVisitor)
    }
}
