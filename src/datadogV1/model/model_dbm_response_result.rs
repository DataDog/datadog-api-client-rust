// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result data container for a DBM query response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DbmResponseResult {
    /// Total number of events matching the request.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Array of matching DBM events. Each event is a free-form object whose fields vary by database type (MySQL, PostgreSQL, SQL Server, etc.).
    #[serde(rename = "events")]
    pub events: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DbmResponseResult {
    pub fn new() -> DbmResponseResult {
        DbmResponseResult {
            count: None,
            events: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn events(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.events = Some(value);
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

impl Default for DbmResponseResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DbmResponseResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DbmResponseResultVisitor;
        impl<'a> Visitor<'a> for DbmResponseResultVisitor {
            type Value = DbmResponseResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut events: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DbmResponseResult {
                    count,
                    events,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DbmResponseResultVisitor)
    }
}
