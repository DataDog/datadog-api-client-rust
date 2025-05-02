// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List relation response links.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListRelationCatalogResponseLinks {
    /// Next link.
    #[serde(rename = "next")]
    pub next: Option<String>,
    /// Previous link.
    #[serde(rename = "previous")]
    pub previous: Option<String>,
    /// Current link.
    #[serde(rename = "self")]
    pub self_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListRelationCatalogResponseLinks {
    pub fn new() -> ListRelationCatalogResponseLinks {
        ListRelationCatalogResponseLinks {
            next: None,
            previous: None,
            self_: None,
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

impl Default for ListRelationCatalogResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListRelationCatalogResponseLinks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListRelationCatalogResponseLinksVisitor;
        impl<'a> Visitor<'a> for ListRelationCatalogResponseLinksVisitor {
            type Value = ListRelationCatalogResponseLinks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
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

                let content = ListRelationCatalogResponseLinks {
                    next,
                    previous,
                    self_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListRelationCatalogResponseLinksVisitor)
    }
}
