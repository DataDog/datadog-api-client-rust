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
pub struct FormulaDatasetQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Vec<crate::datadogV1_20270101::model::DatasetCompute>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaDatasetQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dataset_provider")]
    pub dataset_provider: crate::datadogV1_20270101::model::PublishedDatasetProvider,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::DatasetGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<Vec<crate::datadogV1_20270101::model::DatasetSort>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_column")]
    pub time_column: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaDatasetQuery {
    pub fn new(compute: Vec<crate::datadogV1_20270101::model::DatasetCompute>, data_source: crate::datadogV1_20270101::model::FormulaDatasetQueryDataSource, dataset_id: String, dataset_provider: crate::datadogV1_20270101::model::PublishedDatasetProvider, name: String) -> FormulaDatasetQuery {
        FormulaDatasetQuery {
            compute,
            data_source,
            dataset_id,
            dataset_provider,
            group_by: None,
            limit: None,
            name,
            query_filter: None,
            sort: None,
            time_column: None,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::DatasetGroupBy>) -> Self {
        self.group_by = Some(value);
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
    
    pub fn sort(mut self, value: Vec<crate::datadogV1_20270101::model::DatasetSort>) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn time_column(mut self, value: String) -> Self {
        self.time_column = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaDatasetQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaDatasetQueryVisitor;
        impl<'a> Visitor<'a> for FormulaDatasetQueryVisitor {
            type Value = FormulaDatasetQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<Vec<crate::datadogV1_20270101::model::DatasetCompute>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaDatasetQueryDataSource> = None;
                let mut dataset_id: Option<String> = None;
                let mut dataset_provider: Option<crate::datadogV1_20270101::model::PublishedDatasetProvider> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::DatasetGroupBy>> = None;
                let mut limit: Option<f64> = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut sort: Option<Vec<crate::datadogV1_20270101::model::DatasetSort>> = None;
                let mut time_column: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaDatasetQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "dataset_provider" => {
                            dataset_provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dataset_provider) = dataset_provider {
                                match _dataset_provider {
                                    crate::datadogV1_20270101::model::PublishedDatasetProvider::UnparsedObject(_dataset_provider) => {
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
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        },
                        "time_column" => {
                            if v.is_null() {
                                continue;
                            }
                            time_column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let dataset_provider = dataset_provider.ok_or_else(|| M::Error::missing_field("dataset_provider"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FormulaDatasetQuery {
                    compute,
                    data_source,
                    dataset_id,
                    dataset_provider,
                    group_by,
                    limit,
                    name,
                    query_filter,
                    sort,
                    time_column,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaDatasetQueryVisitor)
    }
}
