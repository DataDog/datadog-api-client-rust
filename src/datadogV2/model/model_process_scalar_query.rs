// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A query for host-level process metrics such as CPU and memory usage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProcessScalarQuery {
    /// The type of aggregation that can be performed on metrics-based queries.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV2::model::MetricsAggregator>,
    /// A data source for process-level infrastructure metrics.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::ProcessDataSource,
    /// Whether CPU metrics should be normalized by core count.
    #[serde(rename = "is_normalized_cpu")]
    pub is_normalized_cpu: Option<bool>,
    /// Maximum number of results to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The process metric to query.
    #[serde(rename = "metric")]
    pub metric: String,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Direction of sort.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::QuerySortOrder>,
    /// Tag filters to narrow down processes.
    #[serde(rename = "tag_filters")]
    pub tag_filters: Option<Vec<String>>,
    /// A full-text search filter to match process names or commands.
    #[serde(rename = "text_filter")]
    pub text_filter: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProcessScalarQuery {
    pub fn new(
        data_source: crate::datadogV2::model::ProcessDataSource,
        metric: String,
        name: String,
    ) -> ProcessScalarQuery {
        ProcessScalarQuery {
            aggregator: None,
            data_source,
            is_normalized_cpu: None,
            limit: None,
            metric,
            name,
            sort: None,
            tag_filters: None,
            text_filter: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregator(mut self, value: crate::datadogV2::model::MetricsAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }

    pub fn is_normalized_cpu(mut self, value: bool) -> Self {
        self.is_normalized_cpu = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::QuerySortOrder) -> Self {
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProcessScalarQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProcessScalarQueryVisitor;
        impl<'a> Visitor<'a> for ProcessScalarQueryVisitor {
            type Value = ProcessScalarQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV2::model::MetricsAggregator> = None;
                let mut data_source: Option<crate::datadogV2::model::ProcessDataSource> = None;
                let mut is_normalized_cpu: Option<bool> = None;
                let mut limit: Option<i64> = None;
                let mut metric: Option<String> = None;
                let mut name: Option<String> = None;
                let mut sort: Option<crate::datadogV2::model::QuerySortOrder> = None;
                let mut tag_filters: Option<Vec<String>> = None;
                let mut text_filter: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV2::model::MetricsAggregator::UnparsedObject(
                                        _aggregator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::ProcessDataSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "is_normalized_cpu" => {
                            if v.is_null() {
                                continue;
                            }
                            is_normalized_cpu =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV2::model::QuerySortOrder::UnparsedObject(
                                        _sort,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            text_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ProcessScalarQuery {
                    aggregator,
                    data_source,
                    is_normalized_cpu,
                    limit,
                    metric,
                    name,
                    sort,
                    tag_filters,
                    text_filter,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProcessScalarQueryVisitor)
    }
}
