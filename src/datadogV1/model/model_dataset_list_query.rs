// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query that lists the rows of a published dataset (a DDSQL query) without aggregation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatasetListQuery {
    /// Identifies this as a published-dataset list query.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::DatasetListQueryDataSourceType,
    /// ID of the published dataset to query.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Product page that published the dataset queried by a `DatasetListQuery`. `ddsql_query` is the only provider currently supported for host map widgets.
    #[serde(rename = "dataset_provider")]
    pub dataset_provider: crate::datadogV1::model::PublishedDatasetProvider,
    /// Filter applied to the dataset's rows, using events-style search syntax.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Maximum number of rows to return from the dataset query.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Sort configuration for a `DatasetListQuery`.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::DatasetListQuerySort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatasetListQuery {
    pub fn new(
        data_source: crate::datadogV1::model::DatasetListQueryDataSourceType,
        dataset_id: String,
        dataset_provider: crate::datadogV1::model::PublishedDatasetProvider,
    ) -> DatasetListQuery {
        DatasetListQuery {
            data_source,
            dataset_id,
            dataset_provider,
            filter: None,
            limit: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV1::model::DatasetListQuerySort) -> Self {
        self.sort = Some(value);
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
                let mut data_source: Option<
                    crate::datadogV1::model::DatasetListQueryDataSourceType,
                > = None;
                let mut dataset_id: Option<String> = None;
                let mut dataset_provider: Option<
                    crate::datadogV1::model::PublishedDatasetProvider,
                > = None;
                let mut filter: Option<String> = None;
                let mut limit: Option<i64> = None;
                let mut sort: Option<crate::datadogV1::model::DatasetListQuerySort> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::DatasetListQueryDataSourceType::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_provider" => {
                            dataset_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dataset_provider) = dataset_provider {
                                match _dataset_provider {
                                    crate::datadogV1::model::PublishedDatasetProvider::UnparsedObject(_dataset_provider) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let dataset_provider =
                    dataset_provider.ok_or_else(|| M::Error::missing_field("dataset_provider"))?;

                let content = DatasetListQuery {
                    data_source,
                    dataset_id,
                    dataset_provider,
                    filter,
                    limit,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetListQueryVisitor)
    }
}
