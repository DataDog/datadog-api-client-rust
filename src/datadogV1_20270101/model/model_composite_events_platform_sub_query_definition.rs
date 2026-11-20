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
pub struct CompositeEventsPlatformSubQueryDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1_20270101::model::ListColumn>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Vec<crate::datadogV1_20270101::model::AggregateCompute>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute_count")]
    pub compute_count: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1_20270101::model::CompositeEventsPlatformSubQueryDefinitionSearch>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "storage")]
    pub storage: Option<crate::datadogV1_20270101::model::StorageType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CompositeEventsPlatformSubQueryDefinition {
    pub fn new(compute: Vec<crate::datadogV1_20270101::model::AggregateCompute>, data_source: crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource, group_by: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>, name: String) -> CompositeEventsPlatformSubQueryDefinition {
        CompositeEventsPlatformSubQueryDefinition {
            columns: None,
            compute,
            compute_count: None,
            data_source,
            group_by,
            indexes: None,
            name,
            search: None,
            storage: None,
            _unparsed: false,
        }
    }
    
    pub fn columns(mut self, value: Vec<crate::datadogV1_20270101::model::ListColumn>) -> Self {
        self.columns = Some(value);
        self
    }
    
    pub fn compute_count(mut self, value: bool) -> Self {
        self.compute_count = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::CompositeEventsPlatformSubQueryDefinitionSearch) -> Self {
        self.search = Some(value);
        self
    }
    
    pub fn storage(mut self, value: crate::datadogV1_20270101::model::StorageType) -> Self {
        self.storage = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for CompositeEventsPlatformSubQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompositeEventsPlatformSubQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for CompositeEventsPlatformSubQueryDefinitionVisitor {
            type Value = CompositeEventsPlatformSubQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::ListColumn>> = None;
                let mut compute: Option<Vec<crate::datadogV1_20270101::model::AggregateCompute>> = None;
                let mut compute_count: Option<bool> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1_20270101::model::CompositeEventsPlatformSubQueryDefinitionSearch> = None;
                let mut storage: Option<crate::datadogV1_20270101::model::StorageType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "compute_count" => {
                            if v.is_null() {
                                continue;
                            }
                            compute_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CompositeEventsPlatformSubQueryDefinition {
                    columns,
                    compute,
                    compute_count,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    search,
                    storage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CompositeEventsPlatformSubQueryDefinitionVisitor)
    }
}
