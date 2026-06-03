// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Page information for the source maps list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SourcemapsListMetaPage {
    /// Whether there are more results available beyond the current page.
    #[serde(rename = "has_more_results")]
    pub has_more_results: bool,
    /// Total number of source maps matching the filter criteria.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SourcemapsListMetaPage {
    pub fn new(has_more_results: bool, total_filtered_count: i64) -> SourcemapsListMetaPage {
        SourcemapsListMetaPage {
            has_more_results,
            total_filtered_count,
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

impl<'de> Deserialize<'de> for SourcemapsListMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SourcemapsListMetaPageVisitor;
        impl<'a> Visitor<'a> for SourcemapsListMetaPageVisitor {
            type Value = SourcemapsListMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_more_results: Option<bool> = None;
                let mut total_filtered_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_more_results" => {
                            has_more_results =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered_count" => {
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
                let has_more_results =
                    has_more_results.ok_or_else(|| M::Error::missing_field("has_more_results"))?;
                let total_filtered_count = total_filtered_count
                    .ok_or_else(|| M::Error::missing_field("total_filtered_count"))?;

                let content = SourcemapsListMetaPage {
                    has_more_results,
                    total_filtered_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SourcemapsListMetaPageVisitor)
    }
}
