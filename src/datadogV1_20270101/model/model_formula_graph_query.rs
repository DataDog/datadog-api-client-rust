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
pub struct FormulaGraphQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::FormulaGraphQueryCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaGraphQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaGraphQueryGroupByItems>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: crate::datadogV1_20270101::model::GraphQuerySearch,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "semantic_mode")]
    pub semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "storage")]
    pub storage: Option<crate::datadogV1_20270101::model::StorageType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaGraphQuery {
    pub fn new(compute: crate::datadogV1_20270101::model::FormulaGraphQueryCompute, data_source: crate::datadogV1_20270101::model::FormulaGraphQueryDataSource, name: String, search: crate::datadogV1_20270101::model::GraphQuerySearch) -> FormulaGraphQuery {
        FormulaGraphQuery {
            compute,
            data_source,
            group_by: None,
            name,
            search,
            semantic_mode: None,
            storage: None,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::FormulaGraphQueryGroupByItems>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn semantic_mode(mut self, value: crate::datadogV1_20270101::model::FormulaSemanticMode) -> Self {
        self.semantic_mode = Some(value);
        self
    }
    
    pub fn storage(mut self, value: crate::datadogV1_20270101::model::StorageType) -> Self {
        self.storage = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaGraphQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaGraphQueryVisitor;
        impl<'a> Visitor<'a> for FormulaGraphQueryVisitor {
            type Value = FormulaGraphQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1_20270101::model::FormulaGraphQueryCompute> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaGraphQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaGraphQueryGroupByItems>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1_20270101::model::GraphQuerySearch> = None;
                let mut semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode> = None;
                let mut storage: Option<crate::datadogV1_20270101::model::StorageType> = None;
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
                                    crate::datadogV1_20270101::model::FormulaGraphQueryDataSource::UnparsedObject(_data_source) => {
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "semantic_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            semantic_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _semantic_mode) = semantic_mode {
                                match _semantic_mode {
                                    crate::datadogV1_20270101::model::FormulaSemanticMode::UnparsedObject(_semantic_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = FormulaGraphQuery {
                    compute,
                    data_source,
                    group_by,
                    name,
                    search,
                    semantic_mode,
                    storage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaGraphQueryVisitor)
    }
}
