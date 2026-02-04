// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregations of notebook search results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookSearchAggregations {
    /// Aggregation by author.
    #[serde(rename = "author")]
    pub author: Option<Vec<crate::datadogV2::model::NotebookSearchAggregationBucketMultiKey>>,
    /// Aggregation by tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>>,
    /// Aggregation by template variable names.
    #[serde(rename = "template_variables.name")]
    pub template_variables_name:
        Option<Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>>,
    /// Aggregation by notebook type.
    #[serde(rename = "type")]
    pub type_: Option<Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookSearchAggregations {
    pub fn new() -> NotebookSearchAggregations {
        NotebookSearchAggregations {
            author: None,
            tags: None,
            template_variables_name: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(
        mut self,
        value: Vec<crate::datadogV2::model::NotebookSearchAggregationBucketMultiKey>,
    ) -> Self {
        self.author = Some(value);
        self
    }

    pub fn tags(
        mut self,
        value: Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>,
    ) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn template_variables_name(
        mut self,
        value: Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>,
    ) -> Self {
        self.template_variables_name = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>,
    ) -> Self {
        self.type_ = Some(value);
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

impl Default for NotebookSearchAggregations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NotebookSearchAggregations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookSearchAggregationsVisitor;
        impl<'a> Visitor<'a> for NotebookSearchAggregationsVisitor {
            type Value = NotebookSearchAggregations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<
                    Vec<crate::datadogV2::model::NotebookSearchAggregationBucketMultiKey>,
                > = None;
                let mut tags: Option<
                    Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>,
                > = None;
                let mut template_variables_name: Option<
                    Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>,
                > = None;
                let mut type_: Option<
                    Vec<crate::datadogV2::model::NotebookSearchAggregationBucketKey>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables.name" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NotebookSearchAggregations {
                    author,
                    tags,
                    template_variables_name,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookSearchAggregationsVisitor)
    }
}
