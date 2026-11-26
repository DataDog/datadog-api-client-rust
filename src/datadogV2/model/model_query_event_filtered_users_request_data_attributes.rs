// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for filtering users by both user properties and event platform activity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QueryEventFilteredUsersRequestDataAttributes {
    /// Event platform query used to filter users based on their event activity within a specified time window.
    #[serde(rename = "event_query")]
    pub event_query:
        Option<crate::datadogV2::model::QueryEventFilteredUsersRequestDataAttributesEventQuery>,
    /// Whether to include the total count of matching users in the response.
    #[serde(rename = "include_row_count")]
    pub include_row_count: Option<bool>,
    /// Maximum number of user records to return in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Filter expression on user profile attributes, such as `first_country_code:US`.
    /// It uses the same attribute names as `select_columns`.
    /// Filter timestamp attributes such as `first_seen` or `last_seen` with Unix timestamps in milliseconds, for
    /// example
    /// `first_seen:>=1793750400000` or `first_seen:[1793750400000 TO 1794355200000]`, or with offsets in
    /// milliseconds
    /// relative to now, for example `first_seen:[-604800000 TO 0]` for the last 7 days. A range needs an upper
    /// bound.
    /// Date strings such as `2026-10-26T00:00:00Z` are not supported, because their colons break the query syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// User profile attribute names to include in the response, such as `user_id`, `user_email`, `user_name`,
    /// `first_seen`, `last_seen`, or `first_country_code`. These are profile attributes, not event facets or
    /// computed metrics. An unknown name returns a 400 error. Use the get mapping endpoint with entity `users` to
    /// list the
    /// attributes available in your organization. Omit it to return every attribute.
    #[serde(rename = "select_columns")]
    pub select_columns: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl QueryEventFilteredUsersRequestDataAttributes {
    pub fn new() -> QueryEventFilteredUsersRequestDataAttributes {
        QueryEventFilteredUsersRequestDataAttributes {
            event_query: None,
            include_row_count: None,
            limit: None,
            query: None,
            select_columns: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn event_query(
        mut self,
        value: crate::datadogV2::model::QueryEventFilteredUsersRequestDataAttributesEventQuery,
    ) -> Self {
        self.event_query = Some(value);
        self
    }

    pub fn include_row_count(mut self, value: bool) -> Self {
        self.include_row_count = Some(value);
        self
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for QueryEventFilteredUsersRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for QueryEventFilteredUsersRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryEventFilteredUsersRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for QueryEventFilteredUsersRequestDataAttributesVisitor {
            type Value = QueryEventFilteredUsersRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_query: Option<
                    crate::datadogV2::model::QueryEventFilteredUsersRequestDataAttributesEventQuery,
                > = None;
                let mut include_row_count: Option<bool> = None;
                let mut limit: Option<i64> = None;
                let mut query: Option<String> = None;
                let mut select_columns: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_query" => {
                            if v.is_null() {
                                continue;
                            }
                            event_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_row_count" => {
                            if v.is_null() {
                                continue;
                            }
                            include_row_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = QueryEventFilteredUsersRequestDataAttributes {
                    event_query,
                    include_row_count,
                    limit,
                    query,
                    select_columns,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryEventFilteredUsersRequestDataAttributesVisitor)
    }
}
