// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CasesResponseMetaPagination {
    /// Current page number
    #[serde(rename = "current")]
    pub current: Option<i64>,
    /// Number of cases in current page
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// Total number of pages
    #[serde(rename = "total")]
    pub total: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CasesResponseMetaPagination {
    pub fn new() -> CasesResponseMetaPagination {
        CasesResponseMetaPagination {
            current: None,
            size: None,
            total: None,
            _unparsed: false,
        }
    }

    pub fn current(mut self, value: i64) -> Self {
        self.current = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }
}

impl Default for CasesResponseMetaPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CasesResponseMetaPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CasesResponseMetaPaginationVisitor;
        impl<'a> Visitor<'a> for CasesResponseMetaPaginationVisitor {
            type Value = CasesResponseMetaPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current: Option<i64> = None;
                let mut size: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current" => {
                            if v.is_null() {
                                continue;
                            }
                            current = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CasesResponseMetaPagination {
                    current,
                    size,
                    total,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CasesResponseMetaPaginationVisitor)
    }
}
