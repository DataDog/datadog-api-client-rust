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
pub struct ProcessQueryDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filter_by")]
    pub filter_by: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_normalized_cpu")]
    pub is_normalized_cpu: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search_by")]
    pub search_by: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ProcessQueryDefinition {
    pub fn new(limit: f64, metric: String) -> ProcessQueryDefinition {
        ProcessQueryDefinition {
            filter_by: None,
            is_normalized_cpu: None,
            limit,
            metric,
            query_filter: None,
            search_by: None,
            _unparsed: false,
        }
    }
    
    pub fn filter_by(mut self, value: Vec<String>) -> Self {
        self.filter_by = Some(value);
        self
    }
    
    pub fn is_normalized_cpu(mut self, value: bool) -> Self {
        self.is_normalized_cpu = Some(value);
        self
    }
    
    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }
    
    pub fn search_by(mut self, value: String) -> Self {
        self.search_by = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ProcessQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProcessQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for ProcessQueryDefinitionVisitor {
            type Value = ProcessQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter_by: Option<Vec<String>> = None;
                let mut is_normalized_cpu: Option<bool> = None;
                let mut limit: Option<f64> = None;
                let mut metric: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut search_by: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter_by" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "is_normalized_cpu" => {
                            if v.is_null() {
                                continue;
                            }
                            is_normalized_cpu = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "search_by" => {
                            if v.is_null() {
                                continue;
                            }
                            search_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;

                let content = ProcessQueryDefinition {
                    filter_by,
                    is_normalized_cpu,
                    limit,
                    metric,
                    query_filter,
                    search_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProcessQueryDefinitionVisitor)
    }
}
