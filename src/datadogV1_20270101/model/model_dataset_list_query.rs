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
pub struct DatasetListQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::DatasetListQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
	#[serialize_always]
    #[serde(rename = "dataset_id")]
    pub dataset_id: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dataset_provider")]
    pub dataset_provider: crate::datadogV1_20270101::model::PublishedDatasetProvider,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::DatasetListQuerySort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DatasetListQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::DatasetListQueryDataSource, dataset_id: Option<String>, dataset_provider: crate::datadogV1_20270101::model::PublishedDatasetProvider) -> DatasetListQuery {
        DatasetListQuery {
            data_source,
            dataset_id,
            dataset_provider,
            filter: None,
            limit: None,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::DatasetListQuerySort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for DatasetListQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetListQueryVisitor;
        impl<'a> Visitor<'a> for DatasetListQueryVisitor {
            type Value = DatasetListQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::DatasetListQueryDataSource> = None;
                let mut dataset_id: Option<Option<String>> = None;
                let mut dataset_provider: Option<crate::datadogV1_20270101::model::PublishedDatasetProvider> = None;
                let mut filter: Option<String> = None;
                let mut limit: Option<f64> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::DatasetListQuerySort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::DatasetListQueryDataSource::UnparsedObject(_data_source) => {
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
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let dataset_provider = dataset_provider.ok_or_else(|| M::Error::missing_field("dataset_provider"))?;

                let content = DatasetListQuery {
                    data_source,
                    dataset_id,
                    dataset_provider,
                    filter,
                    limit,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetListQueryVisitor)
    }
}
