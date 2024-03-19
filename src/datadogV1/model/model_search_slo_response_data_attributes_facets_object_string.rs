// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facet
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchSLOResponseDataAttributesFacetsObjectString {
    /// Count
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Facet
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchSLOResponseDataAttributesFacetsObjectString {
    pub fn new() -> SearchSLOResponseDataAttributesFacetsObjectString {
        SearchSLOResponseDataAttributesFacetsObjectString {
            count: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for SearchSLOResponseDataAttributesFacetsObjectString {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchSLOResponseDataAttributesFacetsObjectString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchSLOResponseDataAttributesFacetsObjectStringVisitor;
        impl<'a> Visitor<'a> for SearchSLOResponseDataAttributesFacetsObjectStringVisitor {
            type Value = SearchSLOResponseDataAttributesFacetsObjectString;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SearchSLOResponseDataAttributesFacetsObjectString {
                    count,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchSLOResponseDataAttributesFacetsObjectStringVisitor)
    }
}
