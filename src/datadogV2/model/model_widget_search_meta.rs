// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the search results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetSearchMeta {
    /// Total number of widgets created by anyone.
    #[serde(rename = "created_by_anyone_total")]
    pub created_by_anyone_total: Option<i64>,
    /// Total number of widgets created by the current user.
    #[serde(rename = "created_by_you_total")]
    pub created_by_you_total: Option<i64>,
    /// Total number of widgets favorited by the current user.
    #[serde(rename = "favorited_by_you_total")]
    pub favorited_by_you_total: Option<i64>,
    /// Total number of widgets matching the current filter criteria.
    #[serde(rename = "filtered_total")]
    pub filtered_total: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetSearchMeta {
    pub fn new() -> WidgetSearchMeta {
        WidgetSearchMeta {
            created_by_anyone_total: None,
            created_by_you_total: None,
            favorited_by_you_total: None,
            filtered_total: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_anyone_total(mut self, value: i64) -> Self {
        self.created_by_anyone_total = Some(value);
        self
    }

    pub fn created_by_you_total(mut self, value: i64) -> Self {
        self.created_by_you_total = Some(value);
        self
    }

    pub fn favorited_by_you_total(mut self, value: i64) -> Self {
        self.favorited_by_you_total = Some(value);
        self
    }

    pub fn filtered_total(mut self, value: i64) -> Self {
        self.filtered_total = Some(value);
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

impl Default for WidgetSearchMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetSearchMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetSearchMetaVisitor;
        impl<'a> Visitor<'a> for WidgetSearchMetaVisitor {
            type Value = WidgetSearchMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_anyone_total: Option<i64> = None;
                let mut created_by_you_total: Option<i64> = None;
                let mut favorited_by_you_total: Option<i64> = None;
                let mut filtered_total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by_anyone_total" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_anyone_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_you_total" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_you_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "favorited_by_you_total" => {
                            if v.is_null() {
                                continue;
                            }
                            favorited_by_you_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filtered_total" => {
                            if v.is_null() {
                                continue;
                            }
                            filtered_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = WidgetSearchMeta {
                    created_by_anyone_total,
                    created_by_you_total,
                    favorited_by_you_total,
                    filtered_total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetSearchMetaVisitor)
    }
}
