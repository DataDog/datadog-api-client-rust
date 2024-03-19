// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Exclusion filter for the security filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityFilterExclusionFilter {
    /// Exclusion filter name.
    #[serde(rename = "name")]
    pub name: String,
    /// Exclusion filter query. Logs that match this query are excluded from the security filter.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityFilterExclusionFilter {
    pub fn new(name: String, query: String) -> SecurityFilterExclusionFilter {
        SecurityFilterExclusionFilter {
            name,
            query,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SecurityFilterExclusionFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityFilterExclusionFilterVisitor;
        impl<'a> Visitor<'a> for SecurityFilterExclusionFilterVisitor {
            type Value = SecurityFilterExclusionFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = SecurityFilterExclusionFilter {
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityFilterExclusionFilterVisitor)
    }
}
