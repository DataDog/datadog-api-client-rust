// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metadata for the current pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAttributionPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Records to be skipped before beginning to return.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Direction to sort by.
    #[serde(rename = "sort_direction")]
    pub sort_direction: Option<String>,
    /// Field to sort by.
    #[serde(rename = "sort_name")]
    pub sort_name: Option<String>,
    /// Total number of records.
    #[serde(rename = "total_number_of_records")]
    pub total_number_of_records: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAttributionPagination {
    pub fn new() -> UsageAttributionPagination {
        UsageAttributionPagination {
            limit: None,
            offset: None,
            sort_direction: None,
            sort_name: None,
            total_number_of_records: None,
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

    pub fn sort_direction(mut self, value: String) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn sort_name(mut self, value: String) -> Self {
        self.sort_name = Some(value);
        self
    }

    pub fn total_number_of_records(mut self, value: i64) -> Self {
        self.total_number_of_records = Some(value);
        self
    }
}

impl Default for UsageAttributionPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAttributionPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAttributionPaginationVisitor;
        impl<'a> Visitor<'a> for UsageAttributionPaginationVisitor {
            type Value = UsageAttributionPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<i64> = None;
                let mut offset: Option<i64> = None;
                let mut sort_direction: Option<String> = None;
                let mut sort_name: Option<String> = None;
                let mut total_number_of_records: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            if v.is_null() {
                                continue;
                            }
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort_direction" => {
                            if v.is_null() {
                                continue;
                            }
                            sort_direction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort_name" => {
                            if v.is_null() {
                                continue;
                            }
                            sort_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_number_of_records" => {
                            if v.is_null() {
                                continue;
                            }
                            total_number_of_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageAttributionPagination {
                    limit,
                    offset,
                    sort_direction,
                    sort_name,
                    total_number_of_records,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAttributionPaginationVisitor)
    }
}
