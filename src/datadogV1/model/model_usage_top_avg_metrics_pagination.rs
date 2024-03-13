// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metadata for the current pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageTopAvgMetricsPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The cursor to get the next results (if any). To make the next request, use the same parameters and add `next_record_id`.
    #[serde(
        rename = "next_record_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_record_id: Option<Option<String>>,
    /// Total number of records.
    #[serde(
        rename = "total_number_of_records",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_number_of_records: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageTopAvgMetricsPagination {
    pub fn new() -> UsageTopAvgMetricsPagination {
        UsageTopAvgMetricsPagination {
            limit: None,
            next_record_id: None,
            total_number_of_records: None,
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_record_id(mut self, value: Option<String>) -> Self {
        self.next_record_id = Some(value);
        self
    }

    pub fn total_number_of_records(mut self, value: Option<i64>) -> Self {
        self.total_number_of_records = Some(value);
        self
    }
}

impl Default for UsageTopAvgMetricsPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageTopAvgMetricsPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageTopAvgMetricsPaginationVisitor;
        impl<'a> Visitor<'a> for UsageTopAvgMetricsPaginationVisitor {
            type Value = UsageTopAvgMetricsPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<i64> = None;
                let mut next_record_id: Option<Option<String>> = None;
                let mut total_number_of_records: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_record_id" => {
                            next_record_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_number_of_records" => {
                            total_number_of_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageTopAvgMetricsPagination {
                    limit,
                    next_record_id,
                    total_number_of_records,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageTopAvgMetricsPaginationVisitor)
    }
}
