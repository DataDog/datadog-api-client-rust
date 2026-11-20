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
pub struct FormulaDataset {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaDatasetDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Vec<crate::datadogV1_20270101::model::FormulaDatasetFormulasItems>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1_20270101::model::DashboardInlineSchema40>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::FormulaDatasetResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_window")]
    pub time_window: Option<crate::datadogV1_20270101::model::TimeWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaDataset {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaDatasetDataSource, formulas: Vec<crate::datadogV1_20270101::model::FormulaDatasetFormulasItems>, name: String, queries: Vec<crate::datadogV1_20270101::model::DashboardInlineSchema40>, response_format: crate::datadogV1_20270101::model::FormulaDatasetResponseFormat) -> FormulaDataset {
        FormulaDataset {
            data_source,
            formulas,
            name,
            queries,
            response_format,
            time_window: None,
            _unparsed: false,
        }
    }
    
    pub fn time_window(mut self, value: crate::datadogV1_20270101::model::TimeWindow) -> Self {
        self.time_window = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaDatasetVisitor;
        impl<'a> Visitor<'a> for FormulaDatasetVisitor {
            type Value = FormulaDataset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaDatasetDataSource> = None;
                let mut formulas: Option<Vec<crate::datadogV1_20270101::model::FormulaDatasetFormulasItems>> = None;
                let mut name: Option<String> = None;
                let mut queries: Option<Vec<crate::datadogV1_20270101::model::DashboardInlineSchema40>> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::FormulaDatasetResponseFormat> = None;
                let mut time_window: Option<crate::datadogV1_20270101::model::TimeWindow> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaDatasetDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "formulas" => {
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1_20270101::model::FormulaDatasetResponseFormat::UnparsedObject(_response_format) => {
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
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let formulas = formulas.ok_or_else(|| M::Error::missing_field("formulas"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = FormulaDataset {
                    data_source,
                    formulas,
                    name,
                    queries,
                    response_format,
                    time_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaDatasetVisitor)
    }
}
