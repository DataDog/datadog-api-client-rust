// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API links related to pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Links {
    /// First page link.
    #[serde(rename = "first")]
    pub first: String,
    /// Last page link.
    #[serde(rename = "last")]
    pub last: String,
    /// Next page link.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Previous page link.
    #[serde(rename = "previous")]
    pub previous: Option<String>,
    /// Request link.
    #[serde(rename = "self")]
    pub self_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Links {
    pub fn new(first: String, last: String, self_: String) -> Links {
        Links {
            first,
            last,
            next: None,
            previous: None,
            self_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next(mut self, value: String) -> Self {
        self.next = Some(value);
        self
    }

    pub fn previous(mut self, value: String) -> Self {
        self.previous = Some(value);
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

impl<'de> Deserialize<'de> for Links {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LinksVisitor;
        impl<'a> Visitor<'a> for LinksVisitor {
            type Value = Links;

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
                        "previous" => {
                            if v.is_null() {
                                continue;
                            }
                            previous = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "self" => {
                            self_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let first = first.ok_or_else(|| M::Error::missing_field("first"))?;
                let last = last.ok_or_else(|| M::Error::missing_field("last"))?;
                let self_ = self_.ok_or_else(|| M::Error::missing_field("self_"))?;

                let content = Links {
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

        deserializer.deserialize_any(LinksVisitor)
    }
}
