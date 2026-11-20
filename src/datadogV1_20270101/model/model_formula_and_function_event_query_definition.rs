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
pub struct FormulaAndFunctionEventQueryDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "calculated_fields")]
    pub calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "extractions")]
    pub extractions: Option<Vec<crate::datadogV1_20270101::model::EventsDataExtraction>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupByConfig>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionSearch>,
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

impl FormulaAndFunctionEventQueryDefinition {
    pub fn new(compute: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute, data_source: crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource, name: String) -> FormulaAndFunctionEventQueryDefinition {
        FormulaAndFunctionEventQueryDefinition {
            calculated_fields: None,
            compute,
            cross_org_uuids: None,
            data_source,
            extractions: None,
            group_by: None,
            indexes: None,
            name,
            search: None,
            semantic_mode: None,
            storage: None,
            _unparsed: false,
        }
    }
    
    pub fn calculated_fields(mut self, value: Vec<crate::datadogV1_20270101::model::EventsCalculatedField>) -> Self {
        self.calculated_fields = Some(value);
        self
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn extractions(mut self, value: Vec<crate::datadogV1_20270101::model::EventsDataExtraction>) -> Self {
        self.extractions = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupByConfig) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionSearch) -> Self {
        self.search = Some(value);
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



impl<'de> Deserialize<'de> for FormulaAndFunctionEventQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionEventQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionEventQueryDefinitionVisitor {
            type Value = FormulaAndFunctionEventQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>> = None;
                let mut compute: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource> = None;
                let mut extractions: Option<Vec<crate::datadogV1_20270101::model::EventsDataExtraction>> = None;
                let mut group_by: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupByConfig> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionSearch> = None;
                let mut semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode> = None;
                let mut storage: Option<crate::datadogV1_20270101::model::StorageType> = None;
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
                                    crate::datadogV1_20270101::model::FormulaAndFunctionEventsDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "extractions" => {
                            if v.is_null() {
                                continue;
                            }
                            extractions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _group_by) = group_by {
                                match _group_by {
                                    crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupByConfig::UnparsedObject(_group_by) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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

                let content = FormulaAndFunctionEventQueryDefinition {
                    calculated_fields,
                    compute,
                    cross_org_uuids,
                    data_source,
                    extractions,
                    group_by,
                    indexes,
                    name,
                    search,
                    semantic_mode,
                    storage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionEventQueryDefinitionVisitor)
    }
}
