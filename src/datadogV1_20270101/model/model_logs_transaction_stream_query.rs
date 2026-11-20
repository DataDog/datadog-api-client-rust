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
pub struct LogsTransactionStreamQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "calculated_fields")]
    pub calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV1_20270101::model::LogsTransactionCompute>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::LogsTransactionStreamQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "end_query_string")]
    pub end_query_string: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<serde_json::Value>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::LogsTransactionStreamQuery20270101Sort>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "start_query_string")]
    pub start_query_string: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "storage")]
    pub storage: Option<crate::datadogV1_20270101::model::StorageType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV1_20270101::model::LogsTransactionStreamQueryVersion>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LogsTransactionStreamQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::LogsTransactionStreamQueryDataSource, query_string: String) -> LogsTransactionStreamQuery {
        LogsTransactionStreamQuery {
            calculated_fields: None,
            compute: None,
            cross_org_uuids: None,
            data_source,
            end_query_string: None,
            group_by: None,
            indexes: None,
            query_string,
            sort: None,
            start_query_string: None,
            storage: None,
            version: None,
            _unparsed: false,
        }
    }
    
    pub fn calculated_fields(mut self, value: Vec<crate::datadogV1_20270101::model::EventsCalculatedField>) -> Self {
        self.calculated_fields = Some(value);
        self
    }
    
    pub fn compute(mut self, value: Vec<crate::datadogV1_20270101::model::LogsTransactionCompute>) -> Self {
        self.compute = Some(value);
        self
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn end_query_string(mut self, value: String) -> Self {
        self.end_query_string = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<serde_json::Value>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::LogsTransactionStreamQuery20270101Sort) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn start_query_string(mut self, value: String) -> Self {
        self.start_query_string = Some(value);
        self
    }
    
    pub fn storage(mut self, value: crate::datadogV1_20270101::model::StorageType) -> Self {
        self.storage = Some(value);
        self
    }
    
    pub fn version(mut self, value: crate::datadogV1_20270101::model::LogsTransactionStreamQueryVersion) -> Self {
        self.version = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for LogsTransactionStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsTransactionStreamQueryVisitor;
        impl<'a> Visitor<'a> for LogsTransactionStreamQueryVisitor {
            type Value = LogsTransactionStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>> = None;
                let mut compute: Option<Vec<crate::datadogV1_20270101::model::LogsTransactionCompute>> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::LogsTransactionStreamQueryDataSource> = None;
                let mut end_query_string: Option<String> = None;
                let mut group_by: Option<Vec<serde_json::Value>> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::LogsTransactionStreamQuery20270101Sort> = None;
                let mut start_query_string: Option<String> = None;
                let mut storage: Option<crate::datadogV1_20270101::model::StorageType> = None;
                let mut version: Option<crate::datadogV1_20270101::model::LogsTransactionStreamQueryVersion> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "calculated_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            calculated_fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::LogsTransactionStreamQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "end_query_string" => {
                            if v.is_null() {
                                continue;
                            }
                            end_query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query_string" => {
                            query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV1_20270101::model::LogsTransactionStreamQuery20270101Sort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "start_query_string" => {
                            if v.is_null() {
                                continue;
                            }
                            start_query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "storage" => {
                            if v.is_null() {
                                continue;
                            }
                            storage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _storage) = storage {
                                match _storage {
                                    crate::datadogV1_20270101::model::StorageType::UnparsedObject(_storage) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _version) = version {
                                match _version {
                                    crate::datadogV1_20270101::model::LogsTransactionStreamQueryVersion::UnparsedObject(_version) => {
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
                let query_string = query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = LogsTransactionStreamQuery {
                    calculated_fields,
                    compute,
                    cross_org_uuids,
                    data_source,
                    end_query_string,
                    group_by,
                    indexes,
                    query_string,
                    sort,
                    start_query_string,
                    storage,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsTransactionStreamQueryVisitor)
    }
}
