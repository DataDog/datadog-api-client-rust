// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Page metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationSimpleSearchMetaPage {
    /// Current page number.
    #[serde(rename = "current")]
    pub current: Option<i32>,
    /// Page size used for this response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// Total number of matching results (capped at the maximum search limit).
    #[serde(rename = "total_count")]
    pub total_count: Option<i32>,
    /// Total number of pages available.
    #[serde(rename = "total_pages")]
    pub total_pages: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationSimpleSearchMetaPage {
    pub fn new() -> LLMObsExperimentationSimpleSearchMetaPage {
        LLMObsExperimentationSimpleSearchMetaPage {
            current: None,
            limit: None,
            total_count: None,
            total_pages: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn current(mut self, value: i32) -> Self {
        self.current = Some(value);
        self
    }

    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn total_count(mut self, value: i32) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i32) -> Self {
        self.total_pages = Some(value);
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

impl Default for LLMObsExperimentationSimpleSearchMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentationSimpleSearchMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationSimpleSearchMetaPageVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationSimpleSearchMetaPageVisitor {
            type Value = LLMObsExperimentationSimpleSearchMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current: Option<i32> = None;
                let mut limit: Option<i32> = None;
                let mut total_count: Option<i32> = None;
                let mut total_pages: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current" => {
                            if v.is_null() {
                                continue;
                            }
                            current = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_pages" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = LLMObsExperimentationSimpleSearchMetaPage {
                    current,
                    limit,
                    total_count,
                    total_pages,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationSimpleSearchMetaPageVisitor)
    }
}
