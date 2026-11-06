// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Archived-session query. A window wider than configured archive retention is narrowed before searching.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumArchiveSessionsSearchAttributes {
    /// Opaque next-page cursor. Keep the query, time range, and hide_retained value unchanged.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// Start of the range in Unix milliseconds. Must be positive and earlier than to.
    #[serde(rename = "from")]
    pub from: i64,
    /// When true, exclude sessions already present in hot RUM storage. When false, those sessions can appear in the results. Keep this value unchanged when continuing with a cursor.
    #[serde(rename = "hide_retained")]
    pub hide_retained: Option<bool>,
    /// Page size, at most 1000 (default 50). Values at or below zero use 50.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// RUM search query. Must not be empty.
    #[serde(rename = "query")]
    pub query: String,
    /// End of the range in Unix milliseconds.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumArchiveSessionsSearchAttributes {
    pub fn new(from: i64, query: String, to: i64) -> RumArchiveSessionsSearchAttributes {
        RumArchiveSessionsSearchAttributes {
            cursor: None,
            from,
            hide_retained: None,
            limit: None,
            query,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cursor(mut self, value: String) -> Self {
        self.cursor = Some(value);
        self
    }

    pub fn hide_retained(mut self, value: bool) -> Self {
        self.hide_retained = Some(value);
        self
    }

    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
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

impl<'de> Deserialize<'de> for RumArchiveSessionsSearchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumArchiveSessionsSearchAttributesVisitor;
        impl<'a> Visitor<'a> for RumArchiveSessionsSearchAttributesVisitor {
            type Value = RumArchiveSessionsSearchAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cursor: Option<String> = None;
                let mut from: Option<i64> = None;
                let mut hide_retained: Option<bool> = None;
                let mut limit: Option<i32> = None;
                let mut query: Option<String> = None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            cursor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hide_retained" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_retained =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = RumArchiveSessionsSearchAttributes {
                    cursor,
                    from,
                    hide_retained,
                    limit,
                    query,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumArchiveSessionsSearchAttributesVisitor)
    }
}
