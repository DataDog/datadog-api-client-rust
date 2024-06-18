// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Pagination metadata information for `ListAPIsResponse`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListAPIsResponseMetaPagination {
    /// Number of items in the current page.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Offset for pagination.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Total number of items.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ListAPIsResponseMetaPagination {
    pub fn new() -> ListAPIsResponseMetaPagination {
        ListAPIsResponseMetaPagination {
            limit: None,
            offset: None,
            total_count: None,
            _unparsed: false,
        }
    }
    
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    
    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }
    
}


impl Default for ListAPIsResponseMetaPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListAPIsResponseMetaPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListAPIsResponseMetaPaginationVisitor;
        impl<'a> Visitor<'a> for ListAPIsResponseMetaPaginationVisitor {
            type Value = ListAPIsResponseMetaPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<i64> = None;
                let mut offset: Option<i64> = None;
                let mut total_count: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "offset" => {
                            if v.is_null() {
                                continue;
                            }
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                        },
                    }
                }

                
                let content = ListAPIsResponseMetaPagination {
                    limit,
                    offset,
                    total_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListAPIsResponseMetaPaginationVisitor)
    }
}
