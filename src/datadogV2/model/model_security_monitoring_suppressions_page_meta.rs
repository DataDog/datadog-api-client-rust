// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSuppressionsPageMeta {
    /// Current page number.
    #[serde(rename = "pageNumber")]
    pub page_number: Option<i64>,
    /// Current page size.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
    /// Total count of suppressions.
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSuppressionsPageMeta {
    pub fn new() -> SecurityMonitoringSuppressionsPageMeta {
        SecurityMonitoringSuppressionsPageMeta {
            page_number: None,
            page_size: None,
            total_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
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

impl Default for SecurityMonitoringSuppressionsPageMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSuppressionsPageMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSuppressionsPageMetaVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSuppressionsPageMetaVisitor {
            type Value = SecurityMonitoringSuppressionsPageMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page_number: Option<i64> = None;
                let mut page_size: Option<i64> = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pageNumber" => {
                            if v.is_null() {
                                continue;
                            }
                            page_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pageSize" => {
                            if v.is_null() {
                                continue;
                            }
                            page_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "totalCount" => {
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

                let content = SecurityMonitoringSuppressionsPageMeta {
                    page_number,
                    page_size,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSuppressionsPageMetaVisitor)
    }
}
