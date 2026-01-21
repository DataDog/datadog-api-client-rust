// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Offset-based pagination schema.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPagesPagination {
    /// Integer representing the offset to fetch the first page of results.
    #[serde(rename = "first_offset")]
    pub first_offset: Option<i64>,
    /// Integer representing the offset to fetch the last page of results.
    #[serde(
        rename = "last_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_offset: Option<Option<i64>>,
    /// Integer representing the number of elements to returned in the results.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Integer representing the index of the first element in the next page of results. Equal to page size added to the current offset.
    #[serde(
        rename = "next_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_offset: Option<Option<i64>>,
    /// Integer representing the index of the first element in the results.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Integer representing the index of the first element in the previous page of results.
    #[serde(
        rename = "prev_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub prev_offset: Option<Option<i64>>,
    /// Integer representing the total number of elements available.
    #[serde(rename = "total", default, with = "::serde_with::rust::double_option")]
    pub total: Option<Option<i64>>,
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::StatusPagesPaginationType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPagesPagination {
    pub fn new() -> StatusPagesPagination {
        StatusPagesPagination {
            first_offset: None,
            last_offset: None,
            limit: None,
            next_offset: None,
            offset: None,
            prev_offset: None,
            total: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn first_offset(mut self, value: i64) -> Self {
        self.first_offset = Some(value);
        self
    }

    pub fn last_offset(mut self, value: Option<i64>) -> Self {
        self.last_offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_offset(mut self, value: Option<i64>) -> Self {
        self.next_offset = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn prev_offset(mut self, value: Option<i64>) -> Self {
        self.prev_offset = Some(value);
        self
    }

    pub fn total(mut self, value: Option<i64>) -> Self {
        self.total = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::StatusPagesPaginationType) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for StatusPagesPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StatusPagesPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPagesPaginationVisitor;
        impl<'a> Visitor<'a> for StatusPagesPaginationVisitor {
            type Value = StatusPagesPagination;

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
                let mut next_offset: Option<Option<i64>> = None;
                let mut offset: Option<i64> = None;
                let mut prev_offset: Option<Option<i64>> = None;
                let mut total: Option<Option<i64>> = None;
                let mut type_: Option<crate::datadogV2::model::StatusPagesPaginationType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            prev_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::StatusPagesPaginationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = StatusPagesPagination {
                    first_offset,
                    last_offset,
                    limit,
                    next_offset,
                    offset,
                    prev_offset,
                    total,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPagesPaginationVisitor)
    }
}
