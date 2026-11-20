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
pub struct PublishedAnalysis {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1_20270101::model::DatasetColumn>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::PublishedAnalysisDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_window")]
    pub time_window: Option<crate::datadogV1_20270101::model::TimeWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl PublishedAnalysis {
    pub fn new(data_source: crate::datadogV1_20270101::model::PublishedAnalysisDataSource, dataset_id: String, name: String) -> PublishedAnalysis {
        PublishedAnalysis {
            columns: None,
            data_source,
            dataset_id,
            name,
            query_filter: None,
            time_window: None,
            _unparsed: false,
        }
    }
    
    pub fn columns(mut self, value: Vec<crate::datadogV1_20270101::model::DatasetColumn>) -> Self {
        self.columns = Some(value);
        self
    }
    
    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }
    
    pub fn time_window(mut self, value: crate::datadogV1_20270101::model::TimeWindow) -> Self {
        self.time_window = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for PublishedAnalysis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublishedAnalysisVisitor;
        impl<'a> Visitor<'a> for PublishedAnalysisVisitor {
            type Value = PublishedAnalysis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::DatasetColumn>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::PublishedAnalysisDataSource> = None;
                let mut dataset_id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut time_window: Option<crate::datadogV1_20270101::model::TimeWindow> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::PublishedAnalysisDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = PublishedAnalysis {
                    columns,
                    data_source,
                    dataset_id,
                    name,
                    query_filter,
                    time_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PublishedAnalysisVisitor)
    }
}
