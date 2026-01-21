// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for pagination
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityEntityRiskScoresMeta {
    /// Current page number (1-indexed)
    #[serde(rename = "pageNumber")]
    pub page_number: i64,
    /// Number of items per page
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    /// Query ID for pagination consistency
    #[serde(rename = "queryId")]
    pub query_id: String,
    /// Total number of entities matching the query
    #[serde(rename = "totalRowCount")]
    pub total_row_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityEntityRiskScoresMeta {
    pub fn new(
        page_number: i64,
        page_size: i64,
        query_id: String,
        total_row_count: i64,
    ) -> SecurityEntityRiskScoresMeta {
        SecurityEntityRiskScoresMeta {
            page_number,
            page_size,
            query_id,
            total_row_count,
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

impl<'de> Deserialize<'de> for SecurityEntityRiskScoresMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityEntityRiskScoresMetaVisitor;
        impl<'a> Visitor<'a> for SecurityEntityRiskScoresMetaVisitor {
            type Value = SecurityEntityRiskScoresMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page_number: Option<i64> = None;
                let mut page_size: Option<i64> = None;
                let mut query_id: Option<String> = None;
                let mut total_row_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pageNumber" => {
                            page_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pageSize" => {
                            page_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queryId" => {
                            query_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "totalRowCount" => {
                            total_row_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let page_number =
                    page_number.ok_or_else(|| M::Error::missing_field("page_number"))?;
                let page_size = page_size.ok_or_else(|| M::Error::missing_field("page_size"))?;
                let query_id = query_id.ok_or_else(|| M::Error::missing_field("query_id"))?;
                let total_row_count =
                    total_row_count.ok_or_else(|| M::Error::missing_field("total_row_count"))?;

                let content = SecurityEntityRiskScoresMeta {
                    page_number,
                    page_size,
                    query_id,
                    total_row_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityEntityRiskScoresMetaVisitor)
    }
}
