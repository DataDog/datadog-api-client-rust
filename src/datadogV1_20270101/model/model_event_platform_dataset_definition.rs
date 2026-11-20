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
pub struct EventPlatformDatasetDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1_20270101::model::DatasetColumn>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::AdvancedEventPlatformSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1_20270101::model::EventPlatformDatasetDefinitionSearch>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "storage")]
    pub storage: Option<crate::datadogV1_20270101::model::StorageType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_window")]
    pub time_window: Option<crate::datadogV1_20270101::model::TimeWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl EventPlatformDatasetDefinition {
    pub fn new(columns: Vec<crate::datadogV1_20270101::model::DatasetColumn>, data_source: crate::datadogV1_20270101::model::AdvancedEventPlatformSource, name: String) -> EventPlatformDatasetDefinition {
        EventPlatformDatasetDefinition {
            columns,
            data_source,
            indexes: None,
            name,
            search: None,
            storage: None,
            time_window: None,
            _unparsed: false,
        }
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn search(mut self, value: crate::datadogV1_20270101::model::EventPlatformDatasetDefinitionSearch) -> Self {
        self.search = Some(value);
        self
    }
    
    pub fn storage(mut self, value: crate::datadogV1_20270101::model::StorageType) -> Self {
        self.storage = Some(value);
        self
    }
    
    pub fn time_window(mut self, value: crate::datadogV1_20270101::model::TimeWindow) -> Self {
        self.time_window = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for EventPlatformDatasetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventPlatformDatasetDefinitionVisitor;
        impl<'a> Visitor<'a> for EventPlatformDatasetDefinitionVisitor {
            type Value = EventPlatformDatasetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1_20270101::model::DatasetColumn>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::AdvancedEventPlatformSource> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1_20270101::model::EventPlatformDatasetDefinitionSearch> = None;
                let mut storage: Option<crate::datadogV1_20270101::model::StorageType> = None;
                let mut time_window: Option<crate::datadogV1_20270101::model::TimeWindow> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::AdvancedEventPlatformSource::UnparsedObject(_data_source) => {
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
                        "time_window" => {
                            if v.is_null() {
                                continue;
                            }
                            time_window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = EventPlatformDatasetDefinition {
                    columns,
                    data_source,
                    indexes,
                    name,
                    search,
                    storage,
                    time_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventPlatformDatasetDefinitionVisitor)
    }
}
