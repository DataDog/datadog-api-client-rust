// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentResponseMetaPagination {
    /// The index of the first element in the next page of results. Equal to page size added to the current offset.
    #[serde(rename = "next_offset")]
    pub next_offset: Option<i64>,
    /// The index of the first element in the results.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Maximum size of pages to return.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentResponseMetaPagination {
    pub fn new() -> IncidentResponseMetaPagination {
        IncidentResponseMetaPagination {
            next_offset: None,
            offset: None,
            size: None,
            _unparsed: false,
        }
    }

    pub fn next_offset(mut self, value: i64) -> Self {
        self.next_offset = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
}

impl Default for IncidentResponseMetaPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentResponseMetaPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentResponseMetaPaginationVisitor;
        impl<'a> Visitor<'a> for IncidentResponseMetaPaginationVisitor {
            type Value = IncidentResponseMetaPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_offset: Option<i64> = None;
                let mut offset: Option<i64> = None;
                let mut size: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_offset" => {
                            if v.is_null() {
                                continue;
                            }
                            next_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            if v.is_null() {
                                continue;
                            }
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentResponseMetaPagination {
                    next_offset,
                    offset,
                    size,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentResponseMetaPaginationVisitor)
    }
}
