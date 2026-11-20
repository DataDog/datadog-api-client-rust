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
pub struct SnowflakeQueryDataset {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::SnowflakeQueryDatasetDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sql_query")]
    pub sql_query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "warehouse")]
    pub warehouse: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SnowflakeQueryDataset {
    pub fn new(data_source: crate::datadogV1_20270101::model::SnowflakeQueryDatasetDataSource, name: String, sql_query: String) -> SnowflakeQueryDataset {
        SnowflakeQueryDataset {
            data_source,
            name,
            sql_query,
            warehouse: None,
            _unparsed: false,
        }
    }
    
    pub fn warehouse(mut self, value: String) -> Self {
        self.warehouse = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for SnowflakeQueryDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeQueryDatasetVisitor;
        impl<'a> Visitor<'a> for SnowflakeQueryDatasetVisitor {
            type Value = SnowflakeQueryDataset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::SnowflakeQueryDatasetDataSource> = None;
                let mut name: Option<String> = None;
                let mut sql_query: Option<String> = None;
                let mut warehouse: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::SnowflakeQueryDatasetDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sql_query" => {
                            sql_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "warehouse" => {
                            if v.is_null() {
                                continue;
                            }
                            warehouse = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let sql_query = sql_query.ok_or_else(|| M::Error::missing_field("sql_query"))?;

                let content = SnowflakeQueryDataset {
                    data_source,
                    name,
                    sql_query,
                    warehouse,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeQueryDatasetVisitor)
    }
}
