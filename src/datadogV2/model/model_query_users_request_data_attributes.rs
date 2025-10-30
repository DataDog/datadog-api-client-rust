// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QueryUsersRequestDataAttributes {
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    #[serde(rename = "query")]
    pub query: Option<String>,
    #[serde(rename = "select_columns")]
    pub select_columns: Option<Vec<String>>,
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::QueryUsersRequestDataAttributesSort>,
    #[serde(rename = "wildcard_search_term")]
    pub wildcard_search_term: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl QueryUsersRequestDataAttributes {
    pub fn new() -> QueryUsersRequestDataAttributes {
        QueryUsersRequestDataAttributes {
            limit: None,
            query: None,
            select_columns: None,
            sort: None,
            wildcard_search_term: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn select_columns(mut self, value: Vec<String>) -> Self {
        self.select_columns = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: crate::datadogV2::model::QueryUsersRequestDataAttributesSort,
    ) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn wildcard_search_term(mut self, value: String) -> Self {
        self.wildcard_search_term = Some(value);
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

impl Default for QueryUsersRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for QueryUsersRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryUsersRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for QueryUsersRequestDataAttributesVisitor {
            type Value = QueryUsersRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<i64> = None;
                let mut query: Option<String> = None;
                let mut select_columns: Option<Vec<String>> = None;
                let mut sort: Option<crate::datadogV2::model::QueryUsersRequestDataAttributesSort> =
                    None;
                let mut wildcard_search_term: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "select_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            select_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "wildcard_search_term" => {
                            if v.is_null() {
                                continue;
                            }
                            wildcard_search_term =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = QueryUsersRequestDataAttributes {
                    limit,
                    query,
                    select_columns,
                    sort,
                    wildcard_search_term,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryUsersRequestDataAttributesVisitor)
    }
}
