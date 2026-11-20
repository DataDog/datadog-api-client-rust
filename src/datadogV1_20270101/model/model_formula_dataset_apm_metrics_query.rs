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
pub struct FormulaDatasetApmMetricsQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "byoc_cluster")]
    pub byoc_cluster: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaDatasetApmMetricsQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "operation_mode")]
    pub operation_mode: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "operation_name")]
    pub operation_name: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "peer_tags")]
    pub peer_tags: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "resource_hash")]
    pub resource_hash: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "span_kind")]
    pub span_kind: Option<crate::datadogV1_20270101::model::SpanKind>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV1_20270101::model::ApmMetricStat,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaDatasetApmMetricsQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaDatasetApmMetricsQueryDataSource, name: String, stat: crate::datadogV1_20270101::model::ApmMetricStat) -> FormulaDatasetApmMetricsQuery {
        FormulaDatasetApmMetricsQuery {
            byoc_cluster: None,
            data_source,
            group_by: None,
            name,
            operation_mode: None,
            operation_name: None,
            peer_tags: None,
            query_filter: None,
            resource_hash: None,
            resource_name: None,
            service: None,
            span_kind: None,
            stat,
            _unparsed: false,
        }
    }
    
    pub fn byoc_cluster(mut self, value: String) -> Self {
        self.byoc_cluster = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn operation_mode(mut self, value: String) -> Self {
        self.operation_mode = Some(value);
        self
    }
    
    pub fn operation_name(mut self, value: String) -> Self {
        self.operation_name = Some(value);
        self
    }
    
    pub fn peer_tags(mut self, value: Vec<String>) -> Self {
        self.peer_tags = Some(value);
        self
    }
    
    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }
    
    pub fn resource_hash(mut self, value: String) -> Self {
        self.resource_hash = Some(value);
        self
    }
    
    pub fn resource_name(mut self, value: String) -> Self {
        self.resource_name = Some(value);
        self
    }
    
    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }
    
    pub fn span_kind(mut self, value: crate::datadogV1_20270101::model::SpanKind) -> Self {
        self.span_kind = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaDatasetApmMetricsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaDatasetApmMetricsQueryVisitor;
        impl<'a> Visitor<'a> for FormulaDatasetApmMetricsQueryVisitor {
            type Value = FormulaDatasetApmMetricsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut byoc_cluster: Option<String> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaDatasetApmMetricsQueryDataSource> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut operation_mode: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut peer_tags: Option<Vec<String>> = None;
                let mut query_filter: Option<String> = None;
                let mut resource_hash: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut span_kind: Option<crate::datadogV1_20270101::model::SpanKind> = None;
                let mut stat: Option<crate::datadogV1_20270101::model::ApmMetricStat> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "byoc_cluster" => {
                            if v.is_null() {
                                continue;
                            }
                            byoc_cluster = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaDatasetApmMetricsQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "operation_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            operation_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "operation_name" => {
                            if v.is_null() {
                                continue;
                            }
                            operation_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "peer_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            peer_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "resource_hash" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "span_kind" => {
                            if v.is_null() {
                                continue;
                            }
                            span_kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _span_kind) = span_kind {
                                match _span_kind {
                                    crate::datadogV1_20270101::model::SpanKind::UnparsedObject(_span_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "stat" => {
                            stat = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _stat) = stat {
                                match _stat {
                                    crate::datadogV1_20270101::model::ApmMetricStat::UnparsedObject(_stat) => {
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
                let stat = stat.ok_or_else(|| M::Error::missing_field("stat"))?;

                let content = FormulaDatasetApmMetricsQuery {
                    byoc_cluster,
                    data_source,
                    group_by,
                    name,
                    operation_mode,
                    operation_name,
                    peer_tags,
                    query_filter,
                    resource_hash,
                    resource_name,
                    service,
                    span_kind,
                    stat,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaDatasetApmMetricsQueryVisitor)
    }
}
