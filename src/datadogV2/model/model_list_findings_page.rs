// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination and findings count information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListFindingsPage {
    /// The cursor used to paginate requests.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// The total count of findings after the filter has been applied.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListFindingsPage {
    pub fn new() -> ListFindingsPage {
        ListFindingsPage {
            cursor: None,
            total_filtered_count: None,
            _unparsed: false,
        }
    }

    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }

    pub fn total_filtered_count(mut self, value: i64) -> Self {
        self.total_filtered_count = Some(value);
        self
    }
}

impl Default for ListFindingsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListFindingsPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListFindingsPageVisitor;
        impl<'a> Visitor<'a> for ListFindingsPageVisitor {
            type Value = ListFindingsPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cursor: Option<String> = None;
                let mut total_filtered_count: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            cursor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_filtered_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ListFindingsPage {
                    cursor,
                    total_filtered_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListFindingsPageVisitor)
    }
}
