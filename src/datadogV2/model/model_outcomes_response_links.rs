// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination links for rule outcomes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesResponseLinks {
    /// Link for the first set of results.
    #[serde(rename = "first")]
    pub first: Option<String>,
    /// Link for the last set of results.
    #[serde(rename = "last")]
    pub last: Option<String>,
    /// Link for the next set of results.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Link for the previous set of results.
    #[serde(rename = "previous")]
    pub previous: Option<String>,
    /// Link for the current set of results.
    #[serde(rename = "self")]
    pub self_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesResponseLinks {
    pub fn new() -> OutcomesResponseLinks {
        OutcomesResponseLinks {
            first: None,
            last: None,
            next: None,
            previous: None,
            self_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn first(mut self, value: String) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: String) -> Self {
        self.last = Some(value);
        self
    }

    pub fn next(mut self, value: String) -> Self {
        self.next = Some(value);
        self
    }

    pub fn previous(mut self, value: String) -> Self {
        self.previous = Some(value);
        self
    }

    pub fn self_(mut self, value: String) -> Self {
        self.self_ = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for OutcomesResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutcomesResponseLinks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesResponseLinksVisitor;
        impl<'a> Visitor<'a> for OutcomesResponseLinksVisitor {
            type Value = OutcomesResponseLinks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first: Option<String> = None;
                let mut last: Option<String> = None;
                let mut next: Option<String> = None;
                let mut previous: Option<String> = None;
                let mut self_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if v.is_null() {
                                continue;
                            }
                            last = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next" => {
                            if v.is_null() {
                                continue;
                            }
                            next = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previous" => {
                            if v.is_null() {
                                continue;
                            }
                            previous = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "self" => {
                            if v.is_null() {
                                continue;
                            }
                            self_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OutcomesResponseLinks {
                    first,
                    last,
                    next,
                    previous,
                    self_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesResponseLinksVisitor)
    }
}
