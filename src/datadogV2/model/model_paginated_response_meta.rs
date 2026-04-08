// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for scores response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PaginatedResponseMeta {
    /// Number of entities in this response.
    #[serde(rename = "count")]
    pub count: i64,
    /// Pagination limit.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Pagination offset.
    #[serde(rename = "offset")]
    pub offset: i64,
    /// Total number of entities available.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PaginatedResponseMeta {
    pub fn new(count: i64, limit: i64, offset: i64, total: i64) -> PaginatedResponseMeta {
        PaginatedResponseMeta {
            count,
            limit,
            offset,
            total,
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

impl<'de> Deserialize<'de> for PaginatedResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PaginatedResponseMetaVisitor;
        impl<'a> Visitor<'a> for PaginatedResponseMetaVisitor {
            type Value = PaginatedResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut limit: Option<i64> = None;
                let mut offset: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let offset = offset.ok_or_else(|| M::Error::missing_field("offset"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = PaginatedResponseMeta {
                    count,
                    limit,
                    offset,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PaginatedResponseMetaVisitor)
    }
}
