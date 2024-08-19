// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The search and filter query settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsQueryFilter {
    /// The minimum time for the requested logs, supports date math and regular timestamps (milliseconds).
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// For customers with multiple indexes, the indexes to search. Defaults to ['*'] which means all indexes.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// The search query - following the log search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Specifies storage type as indexes, online-archives or flex
    #[serde(rename = "storage_tier")]
    pub storage_tier: Option<crate::datadogV2::model::LogsStorageTier>,
    /// The maximum time for the requested logs, supports date math and regular timestamps (milliseconds).
    #[serde(rename = "to")]
    pub to: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsQueryFilter {
    pub fn new() -> LogsQueryFilter {
        LogsQueryFilter {
            from: None,
            indexes: None,
            query: None,
            storage_tier: None,
            to: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from(mut self, value: String) -> Self {
        self.from = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn storage_tier(mut self, value: crate::datadogV2::model::LogsStorageTier) -> Self {
        self.storage_tier = Some(value);
        self
    }

    pub fn to(mut self, value: String) -> Self {
        self.to = Some(value);
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

impl Default for LogsQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsQueryFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsQueryFilterVisitor;
        impl<'a> Visitor<'a> for LogsQueryFilterVisitor {
            type Value = LogsQueryFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<String> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query: Option<String> = None;
                let mut storage_tier: Option<crate::datadogV2::model::LogsStorageTier> = None;
                let mut to: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_tier" => {
                            if v.is_null() {
                                continue;
                            }
                            storage_tier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _storage_tier) = storage_tier {
                                match _storage_tier {
                                    crate::datadogV2::model::LogsStorageTier::UnparsedObject(
                                        _storage_tier,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LogsQueryFilter {
                    from,
                    indexes,
                    query,
                    storage_tier,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsQueryFilterVisitor)
    }
}
