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
pub struct QueryTableColumnLink {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryTableColumnLink {
    pub fn new(url: String) -> QueryTableColumnLink {
        QueryTableColumnLink {
            label: None,
            url,
            _unparsed: false,
        }
    }
    
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryTableColumnLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryTableColumnLinkVisitor;
        impl<'a> Visitor<'a> for QueryTableColumnLinkVisitor {
            type Value = QueryTableColumnLink;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut label: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = QueryTableColumnLink {
                    label,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryTableColumnLinkVisitor)
    }
}
