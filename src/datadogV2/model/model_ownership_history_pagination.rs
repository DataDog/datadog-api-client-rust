// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cursor-based pagination metadata for the history response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipHistoryPagination {
    /// Whether more history entries are available beyond this page.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// An opaque, base64-encoded cursor token. Pass it as the `cursor` query parameter to retrieve the next page. Absent or `null` when there are no further pages.
    #[serde(
        rename = "next_cursor",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_cursor: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipHistoryPagination {
    pub fn new(has_more: bool) -> OwnershipHistoryPagination {
        OwnershipHistoryPagination {
            has_more,
            next_cursor: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_cursor(mut self, value: Option<String>) -> Self {
        self.next_cursor = Some(value);
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

impl<'de> Deserialize<'de> for OwnershipHistoryPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipHistoryPaginationVisitor;
        impl<'a> Visitor<'a> for OwnershipHistoryPaginationVisitor {
            type Value = OwnershipHistoryPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_more: Option<bool> = None;
                let mut next_cursor: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_more" => {
                            has_more = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_cursor" => {
                            next_cursor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let has_more = has_more.ok_or_else(|| M::Error::missing_field("has_more"))?;

                let content = OwnershipHistoryPagination {
                    has_more,
                    next_cursor,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipHistoryPaginationVisitor)
    }
}
