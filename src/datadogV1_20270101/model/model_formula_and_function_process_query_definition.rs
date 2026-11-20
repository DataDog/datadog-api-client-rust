// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionProcessQueryDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaAndFunctionProcessQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_normalized_cpu")]
    pub is_normalized_cpu: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::QuerySortOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tag_filters")]
    pub tag_filters: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_filter")]
    pub text_filter: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaAndFunctionProcessQueryDefinition {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaAndFunctionProcessQueryDataSource, metric: String, name: String) -> FormulaAndFunctionProcessQueryDefinition {
        FormulaAndFunctionProcessQueryDefinition {
            aggregator: None,
            data_source,
            is_normalized_cpu: None,
            limit: None,
            metric,
            name,
            query_filter: None,
            sort: None,
            tag_filters: None,
            text_filter: None,
            _unparsed: false,
        }
    }
    
    pub fn aggregator(mut self, value: crate::datadogV1_20270101::model::ScalarAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }
    
    pub fn is_normalized_cpu(mut self, value: bool) -> Self {
        self.is_normalized_cpu = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::QuerySortOrder) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn tag_filters(mut self, value: Vec<String>) -> Self {
        self.tag_filters = Some(value);
        self
    }
    
    pub fn text_filter(mut self, value: String) -> Self {
        self.text_filter = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaAndFunctionProcessQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionProcessQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionProcessQueryDefinitionVisitor {
            type Value = FormulaAndFunctionProcessQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaAndFunctionProcessQueryDataSource> = None;
                let mut is_normalized_cpu: Option<bool> = None;
                let mut limit: Option<f64> = None;
                let mut metric: Option<String> = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut tag_filters: Option<Vec<String>> = None;
                let mut text_filter: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregator) = aggregator {
                                match _aggregator {
                                    crate::datadogV1_20270101::model::ScalarAggregator::UnparsedObject(_aggregator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaAndFunctionProcessQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "is_normalized_cpu" => {
                            if v.is_null() {
                                continue;
                            }
                            is_normalized_cpu = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV1_20270101::model::QuerySortOrder::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "text_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            text_filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FormulaAndFunctionProcessQueryDefinition {
                    aggregator,
                    data_source,
                    is_normalized_cpu,
                    limit,
                    metric,
                    name,
                    query_filter,
                    sort,
                    tag_filters,
                    text_filter,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionProcessQueryDefinitionVisitor)
    }
}
