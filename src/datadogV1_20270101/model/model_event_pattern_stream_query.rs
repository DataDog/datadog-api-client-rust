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
pub struct EventPatternStreamQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "calculated_fields")]
    pub calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "clustering_pattern_field_path")]
    pub clustering_pattern_field_path: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::EventPatternStreamQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::EventPatternGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::EventPatternStreamQuery20270101Sort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl EventPatternStreamQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::EventPatternStreamQueryDataSource, query_string: String) -> EventPatternStreamQuery {
        EventPatternStreamQuery {
            calculated_fields: None,
            clustering_pattern_field_path: None,
            cross_org_uuids: None,
            data_source,
            group_by: None,
            indexes: None,
            query_string,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn calculated_fields(mut self, value: Vec<crate::datadogV1_20270101::model::EventsCalculatedField>) -> Self {
        self.calculated_fields = Some(value);
        self
    }
    
    pub fn clustering_pattern_field_path(mut self, value: String) -> Self {
        self.clustering_pattern_field_path = Some(value);
        self
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::EventPatternGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::EventPatternStreamQuery20270101Sort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for EventPatternStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventPatternStreamQueryVisitor;
        impl<'a> Visitor<'a> for EventPatternStreamQueryVisitor {
            type Value = EventPatternStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>> = None;
                let mut clustering_pattern_field_path: Option<String> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::EventPatternStreamQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::EventPatternGroupBy>> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::EventPatternStreamQuery20270101Sort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "calculated_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            calculated_fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "clustering_pattern_field_path" => {
                            if v.is_null() {
                                continue;
                            }
                            clustering_pattern_field_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::EventPatternStreamQueryDataSource::UnparsedObject(_data_source) => {
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
                                    crate::datadogV1_20270101::model::EventPatternStreamQuery20270101Sort::UnparsedObject(_sort) => {
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

                let content = EventPatternStreamQuery {
                    calculated_fields,
                    clustering_pattern_field_path,
                    cross_org_uuids,
                    data_source,
                    group_by,
                    indexes,
                    query_string,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventPatternStreamQueryVisitor)
    }
}
