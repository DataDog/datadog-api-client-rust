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
pub struct FormulaChangeTrackingQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaChangeTrackingQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: crate::datadogV1_20270101::model::FormulaChangeTrackingQuerySearch,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaChangeTrackingQuery {
    pub fn new(compute: crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute, data_source: crate::datadogV1_20270101::model::FormulaChangeTrackingQueryDataSource, name: String, search: crate::datadogV1_20270101::model::FormulaChangeTrackingQuerySearch) -> FormulaChangeTrackingQuery {
        FormulaChangeTrackingQuery {
            compute,
            data_source,
            group_by: None,
            name,
            search,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaChangeTrackingQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaChangeTrackingQueryVisitor;
        impl<'a> Visitor<'a> for FormulaChangeTrackingQueryVisitor {
            type Value = FormulaChangeTrackingQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinitionCompute> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaChangeTrackingQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryGroupBy>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1_20270101::model::FormulaChangeTrackingQuerySearch> = None;
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
                                    crate::datadogV1_20270101::model::FormulaChangeTrackingQueryDataSource::UnparsedObject(_data_source) => {
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
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = FormulaChangeTrackingQuery {
                    compute,
                    data_source,
                    group_by,
                    name,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaChangeTrackingQueryVisitor)
    }
}
