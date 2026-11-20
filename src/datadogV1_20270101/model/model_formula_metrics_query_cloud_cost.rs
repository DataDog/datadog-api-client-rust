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
pub struct FormulaMetricsQueryCloudCost {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "byoc_cluster")]
    pub byoc_cluster: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCostDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "semantic_mode")]
    pub semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaMetricsQueryCloudCost {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCostDataSource, name: String, query: String) -> FormulaMetricsQueryCloudCost {
        FormulaMetricsQueryCloudCost {
            aggregator: None,
            byoc_cluster: None,
            cross_org_uuids: None,
            data_source,
            name,
            query,
            semantic_mode: None,
            _unparsed: false,
        }
    }
    
    pub fn aggregator(mut self, value: crate::datadogV1_20270101::model::ScalarAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }
    
    pub fn byoc_cluster(mut self, value: String) -> Self {
        self.byoc_cluster = Some(value);
        self
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn semantic_mode(mut self, value: crate::datadogV1_20270101::model::FormulaSemanticMode) -> Self {
        self.semantic_mode = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaMetricsQueryCloudCost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaMetricsQueryCloudCostVisitor;
        impl<'a> Visitor<'a> for FormulaMetricsQueryCloudCostVisitor {
            type Value = FormulaMetricsQueryCloudCost;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator> = None;
                let mut byoc_cluster: Option<String> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCostDataSource> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode> = None;
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
                        "byoc_cluster" => {
                            if v.is_null() {
                                continue;
                            }
                            byoc_cluster = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cross_org_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_org_uuids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCostDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "semantic_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            semantic_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _semantic_mode) = semantic_mode {
                                match _semantic_mode {
                                    crate::datadogV1_20270101::model::FormulaSemanticMode::UnparsedObject(_semantic_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = FormulaMetricsQueryCloudCost {
                    aggregator,
                    byoc_cluster,
                    cross_org_uuids,
                    data_source,
                    name,
                    query,
                    semantic_mode,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaMetricsQueryCloudCostVisitor)
    }
}
