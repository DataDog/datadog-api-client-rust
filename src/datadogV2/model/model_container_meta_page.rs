// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Paging attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerMetaPage {
    /// The cursor used to get the current results, if any.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// Number of results returned
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The cursor used to get the next results, if any.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// The cursor used to get the previous results, if any.
    #[serde(
        rename = "prev_cursor",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub prev_cursor: Option<Option<String>>,
    /// Total number of records that match the query.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Type of Container pagination.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerMetaPageType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerMetaPage {
    pub fn new() -> ContainerMetaPage {
        ContainerMetaPage {
            cursor: None,
            limit: None,
            next_cursor: None,
            prev_cursor: None,
            total: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }

    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: String) -> Self {
        self.next_cursor = Some(value);
        self
    }

    pub fn prev_cursor(mut self, value: Option<String>) -> Self {
        self.prev_cursor = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ContainerMetaPageType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ContainerMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerMetaPageVisitor;
        impl<'a> Visitor<'a> for ContainerMetaPageVisitor {
            type Value = ContainerMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cursor: Option<String> = None;
                let mut limit: Option<i32> = None;
                let mut next_cursor: Option<String> = None;
                let mut prev_cursor: Option<Option<String>> = None;
                let mut total: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::ContainerMetaPageType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            cursor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            next_cursor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev_cursor" => {
                            prev_cursor =
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
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ContainerMetaPageType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = ContainerMetaPage {
                    cursor,
                    limit,
                    next_cursor,
                    prev_cursor,
                    total,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerMetaPageVisitor)
    }
}
