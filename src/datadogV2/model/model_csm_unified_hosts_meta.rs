// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata for a unified hosts list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmUnifiedHostsMeta {
    /// The current page index (zero-based).
    #[serde(rename = "page_index")]
    pub page_index: i64,
    /// The number of hosts returned per page.
    #[serde(rename = "page_size")]
    pub page_size: i64,
    /// The total number of hosts matching the filter criteria.
    #[serde(rename = "total_filtered")]
    pub total_filtered: i64,
    /// The total number of pages available.
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmUnifiedHostsMeta {
    pub fn new(
        page_index: i64,
        page_size: i64,
        total_filtered: i64,
        total_pages: i64,
    ) -> CsmUnifiedHostsMeta {
        CsmUnifiedHostsMeta {
            page_index,
            page_size,
            total_filtered,
            total_pages,
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

impl<'de> Deserialize<'de> for CsmUnifiedHostsMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmUnifiedHostsMetaVisitor;
        impl<'a> Visitor<'a> for CsmUnifiedHostsMetaVisitor {
            type Value = CsmUnifiedHostsMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page_index: Option<i64> = None;
                let mut page_size: Option<i64> = None;
                let mut total_filtered: Option<i64> = None;
                let mut total_pages: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page_index" => {
                            page_index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_size" => {
                            page_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered" => {
                            total_filtered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_pages" => {
                            total_pages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let page_index = page_index.ok_or_else(|| M::Error::missing_field("page_index"))?;
                let page_size = page_size.ok_or_else(|| M::Error::missing_field("page_size"))?;
                let total_filtered =
                    total_filtered.ok_or_else(|| M::Error::missing_field("total_filtered"))?;
                let total_pages =
                    total_pages.ok_or_else(|| M::Error::missing_field("total_pages"))?;

                let content = CsmUnifiedHostsMeta {
                    page_index,
                    page_size,
                    total_filtered,
                    total_pages,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmUnifiedHostsMetaVisitor)
    }
}
