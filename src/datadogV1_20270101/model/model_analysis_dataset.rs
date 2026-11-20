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
pub struct AnalysisDataset {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::AnalysisDatasetDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::AnalysisDataset20270101Query,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AnalysisDataset {
    pub fn new(data_source: crate::datadogV1_20270101::model::AnalysisDatasetDataSource, name: String, query: crate::datadogV1_20270101::model::AnalysisDataset20270101Query) -> AnalysisDataset {
        AnalysisDataset {
            data_source,
            name,
            query,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for AnalysisDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisDatasetVisitor;
        impl<'a> Visitor<'a> for AnalysisDatasetVisitor {
            type Value = AnalysisDataset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::AnalysisDatasetDataSource> = None;
                let mut name: Option<String> = None;
                let mut query: Option<crate::datadogV1_20270101::model::AnalysisDataset20270101Query> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::AnalysisDatasetDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query) = query {
                                match _query {
                                    crate::datadogV1_20270101::model::AnalysisDataset20270101Query::UnparsedObject(_query) => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = AnalysisDataset {
                    data_source,
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisDatasetVisitor)
    }
}
