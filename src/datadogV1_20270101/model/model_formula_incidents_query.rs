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
pub struct FormulaIncidentsQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaIncidentsQueryDataSource,
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
    pub search: Option<crate::datadogV1_20270101::model::FormulaIncidentsQuerySearch>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaIncidentsQuery {
    pub fn new(compute: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute, data_source: crate::datadogV1_20270101::model::FormulaIncidentsQueryDataSource, name: String) -> FormulaIncidentsQuery {
        FormulaIncidentsQuery {
            compute,
            data_source,
            group_by: None,
            indexes: None,
            name,
            search: None,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::FormulaIncidentsQuerySearch) -> Self {
        self.search = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaIncidentsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaIncidentsQueryVisitor;
        impl<'a> Visitor<'a> for FormulaIncidentsQueryVisitor {
            type Value = FormulaIncidentsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaIncidentsQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1_20270101::model::FormulaIncidentsQuerySearch> = None;
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
                                    crate::datadogV1_20270101::model::FormulaIncidentsQueryDataSource::UnparsedObject(_data_source) => {
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
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FormulaIncidentsQuery {
                    compute,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaIncidentsQueryVisitor)
    }
}
