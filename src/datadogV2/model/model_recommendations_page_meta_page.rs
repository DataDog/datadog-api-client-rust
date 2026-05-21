// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata for a page of cost recommendations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecommendationsPageMetaPage {
    /// The filter expression that was applied to produce this page.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Opaque token used to fetch the next page; absent on the last page.
    #[serde(rename = "next_page_token")]
    pub next_page_token: Option<String>,
    /// Number of items returned in this page (1–10000).
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// Pagination token echoed back from the request.
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RecommendationsPageMetaPage {
    pub fn new() -> RecommendationsPageMetaPage {
        RecommendationsPageMetaPage {
            filter: None,
            next_page_token: None,
            page_size: None,
            page_token: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: String) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
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

impl Default for RecommendationsPageMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RecommendationsPageMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecommendationsPageMetaPageVisitor;
        impl<'a> Visitor<'a> for RecommendationsPageMetaPageVisitor {
            type Value = RecommendationsPageMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<String> = None;
                let mut next_page_token: Option<String> = None;
                let mut page_size: Option<i32> = None;
                let mut page_token: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_page_token" => {
                            if v.is_null() {
                                continue;
                            }
                            next_page_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_size" => {
                            if v.is_null() {
                                continue;
                            }
                            page_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_token" => {
                            if v.is_null() {
                                continue;
                            }
                            page_token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RecommendationsPageMetaPage {
                    filter,
                    next_page_token,
                    page_size,
                    page_token,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RecommendationsPageMetaPageVisitor)
    }
}
