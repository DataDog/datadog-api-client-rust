// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack response pagination metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpacksResponseMetaPagination {
    /// The first offset.
    #[serde(rename = "first_offset")]
    pub first_offset: Option<i64>,
    /// The last offset.
    #[serde(
        rename = "last_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_offset: Option<Option<i64>>,
    /// Pagination limit.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The next offset.
    #[serde(rename = "next_offset")]
    pub next_offset: Option<i64>,
    /// The offset.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// The previous offset.
    #[serde(rename = "prev_offset")]
    pub prev_offset: Option<i64>,
    /// Total results.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Offset type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpacksResponseMetaPagination {
    pub fn new() -> PowerpacksResponseMetaPagination {
        PowerpacksResponseMetaPagination {
            first_offset: None,
            last_offset: None,
            limit: None,
            next_offset: None,
            offset: None,
            prev_offset: None,
            total: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn first_offset(&mut self, value: i64) -> &mut Self {
        self.first_offset = Some(value);
        self
    }

    pub fn last_offset(&mut self, value: Option<i64>) -> &mut Self {
        self.last_offset = Some(value);
        self
    }

    pub fn limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn next_offset(&mut self, value: i64) -> &mut Self {
        self.next_offset = Some(value);
        self
    }

    pub fn offset(&mut self, value: i64) -> &mut Self {
        self.offset = Some(value);
        self
    }

    pub fn prev_offset(&mut self, value: i64) -> &mut Self {
        self.prev_offset = Some(value);
        self
    }

    pub fn total(&mut self, value: i64) -> &mut Self {
        self.total = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for PowerpacksResponseMetaPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PowerpacksResponseMetaPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpacksResponseMetaPaginationVisitor;
        impl<'a> Visitor<'a> for PowerpacksResponseMetaPaginationVisitor {
            type Value = PowerpacksResponseMetaPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first_offset: Option<i64> = None;
                let mut last_offset: Option<Option<i64>> = None;
                let mut limit: Option<i64> = None;
                let mut next_offset: Option<i64> = None;
                let mut offset: Option<i64> = None;
                let mut prev_offset: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first_offset" => {
                            if v.is_null() {
                                continue;
                            }
                            first_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_offset" => {
                            last_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "prev_offset" => {
                            if v.is_null() {
                                continue;
                            }
                            prev_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = PowerpacksResponseMetaPagination {
                    first_offset,
                    last_offset,
                    limit,
                    next_offset,
                    offset,
                    prev_offset,
                    total,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpacksResponseMetaPaginationVisitor)
    }
}
