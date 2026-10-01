// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the rows requested, including which ones were not found.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableRowResourceArrayMeta {
    /// Number of requested rows that were found and returned in `data`.
    #[serde(rename = "found_count")]
    pub found_count: i64,
    /// Row IDs from the request that do not exist in the reference table. Empty when every requested row was found.
    #[serde(rename = "not_found")]
    pub not_found: Vec<String>,
    /// Number of row IDs supplied in the `row_id` query parameter.
    #[serde(rename = "requested_count")]
    pub requested_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableRowResourceArrayMeta {
    pub fn new(
        found_count: i64,
        not_found: Vec<String>,
        requested_count: i64,
    ) -> TableRowResourceArrayMeta {
        TableRowResourceArrayMeta {
            found_count,
            not_found,
            requested_count,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TableRowResourceArrayMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableRowResourceArrayMetaVisitor;
        impl<'a> Visitor<'a> for TableRowResourceArrayMetaVisitor {
            type Value = TableRowResourceArrayMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut found_count: Option<i64> = None;
                let mut not_found: Option<Vec<String>> = None;
                let mut requested_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "found_count" => {
                            found_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "not_found" => {
                            not_found = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requested_count" => {
                            requested_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let found_count =
                    found_count.ok_or_else(|| M::Error::missing_field("found_count"))?;
                let not_found = not_found.ok_or_else(|| M::Error::missing_field("not_found"))?;
                let requested_count =
                    requested_count.ok_or_else(|| M::Error::missing_field("requested_count"))?;

                let content = TableRowResourceArrayMeta {
                    found_count,
                    not_found,
                    requested_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableRowResourceArrayMetaVisitor)
    }
}
