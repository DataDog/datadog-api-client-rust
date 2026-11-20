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
pub struct MetricDatasetMetricScalar {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1_20270101::model::MetricDatasetMetricScalarAggregation,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::MetricDatasetMetricScalarDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metric_name")]
    pub metric_name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tag_names")]
    pub tag_names: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_window")]
    pub time_window: Option<crate::datadogV1_20270101::model::TimeWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl MetricDatasetMetricScalar {
    pub fn new(aggregation: crate::datadogV1_20270101::model::MetricDatasetMetricScalarAggregation, data_source: crate::datadogV1_20270101::model::MetricDatasetMetricScalarDataSource, metric_name: String, name: String) -> MetricDatasetMetricScalar {
        MetricDatasetMetricScalar {
            aggregation,
            data_source,
            metric_name,
            name,
            query_filter: None,
            tag_names: None,
            time_window: None,
            _unparsed: false,
        }
    }
    
    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }
    
    pub fn tag_names(mut self, value: Vec<String>) -> Self {
        self.tag_names = Some(value);
        self
    }
    
    pub fn time_window(mut self, value: crate::datadogV1_20270101::model::TimeWindow) -> Self {
        self.time_window = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for MetricDatasetMetricScalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricDatasetMetricScalarVisitor;
        impl<'a> Visitor<'a> for MetricDatasetMetricScalarVisitor {
            type Value = MetricDatasetMetricScalar;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1_20270101::model::MetricDatasetMetricScalarAggregation> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::MetricDatasetMetricScalarDataSource> = None;
                let mut metric_name: Option<String> = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut tag_names: Option<Vec<String>> = None;
                let mut time_window: Option<crate::datadogV1_20270101::model::TimeWindow> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1_20270101::model::MetricDatasetMetricScalarAggregation::UnparsedObject(_aggregation) => {
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
                                    crate::datadogV1_20270101::model::MetricDatasetMetricScalarDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "metric_name" => {
                            metric_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "tag_names" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_names = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time_window" => {
                            if v.is_null() {
                                continue;
                            }
                            time_window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let aggregation = aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let metric_name = metric_name.ok_or_else(|| M::Error::missing_field("metric_name"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = MetricDatasetMetricScalar {
                    aggregation,
                    data_source,
                    metric_name,
                    name,
                    query_filter,
                    tag_names,
                    time_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricDatasetMetricScalarVisitor)
    }
}
