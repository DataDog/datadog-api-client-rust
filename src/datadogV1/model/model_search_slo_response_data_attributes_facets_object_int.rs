// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facet
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchSLOResponseDataAttributesFacetsObjectInt {
    /// Count
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Facet
    #[serde(rename = "name")]
    pub name: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchSLOResponseDataAttributesFacetsObjectInt {
    pub fn new() -> SearchSLOResponseDataAttributesFacetsObjectInt {
        SearchSLOResponseDataAttributesFacetsObjectInt {
            count: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn name(mut self, value: f64) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for SearchSLOResponseDataAttributesFacetsObjectInt {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchSLOResponseDataAttributesFacetsObjectInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchSLOResponseDataAttributesFacetsObjectIntVisitor;
        impl<'a> Visitor<'a> for SearchSLOResponseDataAttributesFacetsObjectIntVisitor {
            type Value = SearchSLOResponseDataAttributesFacetsObjectInt;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut name: Option<f64> = None;
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

                let content = SearchSLOResponseDataAttributesFacetsObjectInt {
                    count,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchSLOResponseDataAttributesFacetsObjectIntVisitor)
    }
}
