// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the information of a search result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssuesSearchResultAttributes {
    /// Count of sessions impacted by the issue over the queried time window.
    #[serde(rename = "impacted_sessions")]
    pub impacted_sessions: Option<i64>,
    /// Count of users impacted by the issue over the queried time window.
    #[serde(rename = "impacted_users")]
    pub impacted_users: Option<i64>,
    /// Total count of errors that match the issue over the queried time window.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssuesSearchResultAttributes {
    pub fn new() -> IssuesSearchResultAttributes {
        IssuesSearchResultAttributes {
            impacted_sessions: None,
            impacted_users: None,
            total_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn impacted_sessions(mut self, value: i64) -> Self {
        self.impacted_sessions = Some(value);
        self
    }

    pub fn impacted_users(mut self, value: i64) -> Self {
        self.impacted_users = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
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

impl Default for IssuesSearchResultAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssuesSearchResultAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssuesSearchResultAttributesVisitor;
        impl<'a> Visitor<'a> for IssuesSearchResultAttributesVisitor {
            type Value = IssuesSearchResultAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut impacted_sessions: Option<i64> = None;
                let mut impacted_users: Option<i64> = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "impacted_sessions" => {
                            if v.is_null() {
                                continue;
                            }
                            impacted_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impacted_users" => {
                            if v.is_null() {
                                continue;
                            }
                            impacted_users =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssuesSearchResultAttributes {
                    impacted_sessions,
                    impacted_users,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssuesSearchResultAttributesVisitor)
    }
}
