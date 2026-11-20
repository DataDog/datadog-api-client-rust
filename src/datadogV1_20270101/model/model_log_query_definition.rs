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
pub struct LogQueryDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "calculated_fields")]
    pub calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1_20270101::model::LogsQueryCompute>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::LogQueryDefinitionGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "multi_compute")]
    pub multi_compute: Option<Vec<crate::datadogV1_20270101::model::LogsQueryCompute>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1_20270101::model::LogQueryDefinitionSearch>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "storage")]
    pub storage: Option<crate::datadogV1_20270101::model::StorageType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LogQueryDefinition {
    pub fn new() -> LogQueryDefinition {
        LogQueryDefinition {
            calculated_fields: None,
            compute: None,
            group_by: None,
            index: None,
            multi_compute: None,
            search: None,
            storage: None,
            _unparsed: false,
        }
    }
    
    pub fn calculated_fields(mut self, value: Vec<crate::datadogV1_20270101::model::EventsCalculatedField>) -> Self {
        self.calculated_fields = Some(value);
        self
    }
    
    pub fn compute(mut self, value: crate::datadogV1_20270101::model::LogsQueryCompute) -> Self {
        self.compute = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::LogQueryDefinitionGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn index(mut self, value: String) -> Self {
        self.index = Some(value);
        self
    }
    
    pub fn multi_compute(mut self, value: Vec<crate::datadogV1_20270101::model::LogsQueryCompute>) -> Self {
        self.multi_compute = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::LogQueryDefinitionSearch) -> Self {
        self.search = Some(value);
        self
    }
    
    pub fn storage(mut self, value: crate::datadogV1_20270101::model::StorageType) -> Self {
        self.storage = Some(value);
        self
    }
    
}


impl Default for LogQueryDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for LogQueryDefinitionVisitor {
            type Value = LogQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculated_fields: Option<Vec<crate::datadogV1_20270101::model::EventsCalculatedField>> = None;
                let mut compute: Option<crate::datadogV1_20270101::model::LogsQueryCompute> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::LogQueryDefinitionGroupBy>> = None;
                let mut index: Option<String> = None;
                let mut multi_compute: Option<Vec<crate::datadogV1_20270101::model::LogsQueryCompute>> = None;
                let mut search: Option<crate::datadogV1_20270101::model::LogQueryDefinitionSearch> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "index" => {
                            if v.is_null() {
                                continue;
                            }
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "multi_compute" => {
                            if v.is_null() {
                                continue;
                            }
                            multi_compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = LogQueryDefinition {
                    calculated_fields,
                    compute,
                    group_by,
                    index,
                    multi_compute,
                    search,
                    storage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogQueryDefinitionVisitor)
    }
}
