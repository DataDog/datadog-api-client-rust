// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebooksResponsePage {
    /// The total number of notebooks that would be returned if the request was not filtered by `start` and `count` parameters.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    /// The total number of notebooks returned.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebooksResponsePage {
    pub fn new() -> NotebooksResponsePage {
        NotebooksResponsePage {
            total_count: None,
            total_filtered_count: None,
            _unparsed: false,
        }
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_filtered_count(mut self, value: i64) -> Self {
        self.total_filtered_count = Some(value);
        self
    }
}

impl Default for NotebooksResponsePage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NotebooksResponsePage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebooksResponsePageVisitor;
        impl<'a> Visitor<'a> for NotebooksResponsePageVisitor {
            type Value = NotebooksResponsePage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut total_count: Option<i64> = None;
                let mut total_filtered_count: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = NotebooksResponsePage {
                    total_count,
                    total_filtered_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebooksResponsePageVisitor)
    }
}
