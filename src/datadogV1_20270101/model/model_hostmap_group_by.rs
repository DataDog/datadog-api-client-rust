// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostmapGroupBy {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "column")]
    pub column: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapGroupBy {
    pub fn new(column: String) -> HostmapGroupBy {
        HostmapGroupBy {
            column,
            key: None,
            _unparsed: false,
        }
    }
    
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HostmapGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapGroupByVisitor;
        impl<'a> Visitor<'a> for HostmapGroupByVisitor {
            type Value = HostmapGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column: Option<String> = None;
                let mut key: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column" => {
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;

                let content = HostmapGroupBy {
                    column,
                    key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostmapGroupByVisitor)
    }
}
