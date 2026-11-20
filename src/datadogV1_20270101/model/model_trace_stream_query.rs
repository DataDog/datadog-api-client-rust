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
pub struct TraceStreamQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "context")]
    pub context: Option<crate::datadogV1_20270101::model::EventsContext>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::TraceStreamQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "semantic_mode")]
    pub semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::TraceStreamQuery20270101Sort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TraceStreamQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::TraceStreamQueryDataSource, query_string: String) -> TraceStreamQuery {
        TraceStreamQuery {
            context: None,
            cross_org_uuids: None,
            data_source,
            indexes: None,
            query_string,
            semantic_mode: None,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn context(mut self, value: crate::datadogV1_20270101::model::EventsContext) -> Self {
        self.context = Some(value);
        self
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn semantic_mode(mut self, value: crate::datadogV1_20270101::model::FormulaSemanticMode) -> Self {
        self.semantic_mode = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::TraceStreamQuery20270101Sort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TraceStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TraceStreamQueryVisitor;
        impl<'a> Visitor<'a> for TraceStreamQueryVisitor {
            type Value = TraceStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut context: Option<crate::datadogV1_20270101::model::EventsContext> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::TraceStreamQueryDataSource> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut semantic_mode: Option<crate::datadogV1_20270101::model::FormulaSemanticMode> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::TraceStreamQuery20270101Sort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::TraceStreamQueryDataSource::UnparsedObject(_data_source) => {
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
                        "query_string" => {
                            query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV1_20270101::model::TraceStreamQuery20270101Sort::UnparsedObject(_sort) => {
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

                let content = TraceStreamQuery {
                    context,
                    cross_org_uuids,
                    data_source,
                    indexes,
                    query_string,
                    semantic_mode,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TraceStreamQueryVisitor)
    }
}
