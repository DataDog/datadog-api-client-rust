// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorSearchResponseMetadata {
    /// The page to start paginating from.
    #[serde(rename = "page")]
    pub page: Option<i64>,
    /// The number of pages.
    #[serde(rename = "page_count")]
    pub page_count: Option<i64>,
    /// The number of monitors to return per page.
    #[serde(rename = "per_page")]
    pub per_page: Option<i64>,
    /// The total number of monitors.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorSearchResponseMetadata {
    pub fn new() -> MonitorSearchResponseMetadata {
        MonitorSearchResponseMetadata {
            page: None,
            page_count: None,
            per_page: None,
            total_count: None,
            _unparsed: false,
        }
    }

    pub fn page(&mut self, value: i64) -> &mut Self {
        self.page = Some(value);
        self
    }

    pub fn page_count(&mut self, value: i64) -> &mut Self {
        self.page_count = Some(value);
        self
    }

    pub fn per_page(&mut self, value: i64) -> &mut Self {
        self.per_page = Some(value);
        self
    }

    pub fn total_count(&mut self, value: i64) -> &mut Self {
        self.total_count = Some(value);
        self
    }
}

impl Default for MonitorSearchResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorSearchResponseMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorSearchResponseMetadataVisitor;
        impl<'a> Visitor<'a> for MonitorSearchResponseMetadataVisitor {
            type Value = MonitorSearchResponseMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<i64> = None;
                let mut page_count: Option<i64> = None;
                let mut per_page: Option<i64> = None;
                let mut total_count: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_count" => {
                            if v.is_null() {
                                continue;
                            }
                            page_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "per_page" => {
                            if v.is_null() {
                                continue;
                            }
                            per_page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorSearchResponseMetadata {
                    page,
                    page_count,
                    per_page,
                    total_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorSearchResponseMetadataVisitor)
    }
}
