// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parameters for the DBM query samples list query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DbmQuerySamplesRequestList {
    /// Indexes to query. Use `["databasequery"]` for DBM query samples.
    #[serde(rename = "indexes")]
    pub indexes: Vec<String>,
    /// Maximum number of results to return.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// Search query parameters for DBM query samples.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::DbmQuerySamplesSearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DbmQuerySamplesRequestList {
    pub fn new(
        indexes: Vec<String>,
        limit: i32,
        search: crate::datadogV1::model::DbmQuerySamplesSearch,
    ) -> DbmQuerySamplesRequestList {
        DbmQuerySamplesRequestList {
            indexes,
            limit,
            search,
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

impl<'de> Deserialize<'de> for DbmQuerySamplesRequestList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DbmQuerySamplesRequestListVisitor;
        impl<'a> Visitor<'a> for DbmQuerySamplesRequestListVisitor {
            type Value = DbmQuerySamplesRequestList;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut indexes: Option<Vec<String>> = None;
                let mut limit: Option<i32> = None;
                let mut search: Option<crate::datadogV1::model::DbmQuerySamplesSearch> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "indexes" => {
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let indexes = indexes.ok_or_else(|| M::Error::missing_field("indexes"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = DbmQuerySamplesRequestList {
                    indexes,
                    limit,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DbmQuerySamplesRequestListVisitor)
    }
}
