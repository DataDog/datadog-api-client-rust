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
pub struct FormulaCaseQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::CaseCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaCaseQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::CaseGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_filter")]
    pub query_filter: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::QuerySortOrder>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaCaseQuery {
    pub fn new(compute: crate::datadogV1_20270101::model::CaseCompute, data_source: crate::datadogV1_20270101::model::FormulaCaseQueryDataSource, name: String, query_filter: String) -> FormulaCaseQuery {
        FormulaCaseQuery {
            compute,
            data_source,
            group_by: None,
            limit: None,
            name,
            query_filter,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::CaseGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::QuerySortOrder) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaCaseQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaCaseQueryVisitor;
        impl<'a> Visitor<'a> for FormulaCaseQueryVisitor {
            type Value = FormulaCaseQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1_20270101::model::CaseCompute> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaCaseQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::CaseGroupBy>> = None;
                let mut limit: Option<f64> = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
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
                                    crate::datadogV1_20270101::model::FormulaCaseQueryDataSource::UnparsedObject(_data_source) => {
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
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query_filter = query_filter.ok_or_else(|| M::Error::missing_field("query_filter"))?;

                let content = FormulaCaseQuery {
                    compute,
                    data_source,
                    group_by,
                    limit,
                    name,
                    query_filter,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaCaseQueryVisitor)
    }
}
