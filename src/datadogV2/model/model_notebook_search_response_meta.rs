// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the notebook search results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookSearchResponseMeta {
    /// Aggregations of notebook search results.
    #[serde(rename = "aggregations")]
    pub aggregations: Option<crate::datadogV2::model::NotebookSearchAggregations>,
    /// Total number of notebooks found.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookSearchResponseMeta {
    pub fn new(total: i64) -> NotebookSearchResponseMeta {
        NotebookSearchResponseMeta {
            aggregations: None,
            total,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregations(
        mut self,
        value: crate::datadogV2::model::NotebookSearchAggregations,
    ) -> Self {
        self.aggregations = Some(value);
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

impl<'de> Deserialize<'de> for NotebookSearchResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookSearchResponseMetaVisitor;
        impl<'a> Visitor<'a> for NotebookSearchResponseMetaVisitor {
            type Value = NotebookSearchResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregations: Option<crate::datadogV2::model::NotebookSearchAggregations> =
                    None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregations" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = NotebookSearchResponseMeta {
                    aggregations,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookSearchResponseMetaVisitor)
    }
}
