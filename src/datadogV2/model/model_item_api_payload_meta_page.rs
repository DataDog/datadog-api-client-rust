// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ItemApiPayloadMetaPage` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemApiPayloadMetaPage {
    /// The `ItemApiPayloadMeta` `hasMore`.
    #[serde(rename = "hasMore")]
    pub has_more: Option<bool>,
    /// The `ItemApiPayloadMeta` `totalCount`.
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,
    /// The `ItemApiPayloadMeta` `totalFilteredCount`.
    #[serde(rename = "totalFilteredCount")]
    pub total_filtered_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ItemApiPayloadMetaPage {
    pub fn new() -> ItemApiPayloadMetaPage {
        ItemApiPayloadMetaPage {
            has_more: None,
            total_count: None,
            total_filtered_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_filtered_count(mut self, value: i64) -> Self {
        self.total_filtered_count = Some(value);
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

impl Default for ItemApiPayloadMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ItemApiPayloadMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemApiPayloadMetaPageVisitor;
        impl<'a> Visitor<'a> for ItemApiPayloadMetaPageVisitor {
            type Value = ItemApiPayloadMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_more: Option<bool> = None;
                let mut total_count: Option<i64> = None;
                let mut total_filtered_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hasMore" => {
                            if v.is_null() {
                                continue;
                            }
                            has_more = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "totalCount" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "totalFilteredCount" => {
                            if v.is_null() {
                                continue;
                            }
                            total_filtered_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ItemApiPayloadMetaPage {
                    has_more,
                    total_count,
                    total_filtered_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ItemApiPayloadMetaPageVisitor)
    }
}
