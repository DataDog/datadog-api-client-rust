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
pub struct CompositeEventsPlatformQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1_20270101::model::ListColumn>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV1_20270101::model::AggregateCompute>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute_count")]
    pub compute_count: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1_20270101::model::CompositeEventsPlatformQuerySearch>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "storage")]
    pub storage: Option<crate::datadogV1_20270101::model::StorageType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CompositeEventsPlatformQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource, name: String) -> CompositeEventsPlatformQuery {
        CompositeEventsPlatformQuery {
            columns: None,
            compute: None,
            compute_count: None,
            data_source,
            group_by: None,
            indexes: None,
            name,
            search: None,
            storage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn columns(mut self, value: Vec<crate::datadogV1_20270101::model::ListColumn>) -> Self {
        self.columns = Some(value);
        self
    }
    
    pub fn compute(mut self, value: Vec<crate::datadogV1_20270101::model::AggregateCompute>) -> Self {
        self.compute = Some(value);
        self
    }
    
    pub fn compute_count(mut self, value: bool) -> Self {
        self.compute_count = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::CompositeEventsPlatformQuerySearch) -> Self {
        self.search = Some(value);
        self
    }
    
    pub fn storage(mut self, value: crate::datadogV1_20270101::model::StorageType) -> Self {
        self.storage = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for CompositeEventsPlatformQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompositeEventsPlatformQueryVisitor;
        impl<'a> Visitor<'a> for CompositeEventsPlatformQueryVisitor {
            type Value = CompositeEventsPlatformQuery;

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
                let mut search: Option<crate::datadogV1_20270101::model::CompositeEventsPlatformQuerySearch> = None;
                let mut storage: Option<crate::datadogV1_20270101::model::StorageType> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
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
                            if v.is_null() {
                                continue;
                            }
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CompositeEventsPlatformQuery {
                    columns,
                    compute,
                    compute_count,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    search,
                    storage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CompositeEventsPlatformQueryVisitor)
    }
}
