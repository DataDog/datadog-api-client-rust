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
pub struct FormulaDataQualityMetricsQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "anomaly_options")]
    pub anomaly_options: Option<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptions>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filter")]
    pub filter: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "measure")]
    pub measure: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaDataQualityMetricsQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryDataSource, filter: String, measure: String, name: String) -> FormulaDataQualityMetricsQuery {
        FormulaDataQualityMetricsQuery {
            anomaly_options: None,
            data_source,
            filter,
            group_by: None,
            measure,
            name,
            scope: None,
            _unparsed: false,
        }
    }
    
    pub fn anomaly_options(mut self, value: crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptions) -> Self {
        self.anomaly_options = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaDataQualityMetricsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaDataQualityMetricsQueryVisitor;
        impl<'a> Visitor<'a> for FormulaDataQualityMetricsQueryVisitor {
            type Value = FormulaDataQualityMetricsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anomaly_options: Option<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptions> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryDataSource> = None;
                let mut filter: Option<String> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut measure: Option<String> = None;
                let mut name: Option<String> = None;
                let mut scope: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anomaly_options" => {
                            if v.is_null() {
                                continue;
                            }
                            anomaly_options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "measure" => {
                            measure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;
                let measure = measure.ok_or_else(|| M::Error::missing_field("measure"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FormulaDataQualityMetricsQuery {
                    anomaly_options,
                    data_source,
                    filter,
                    group_by,
                    measure,
                    name,
                    scope,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaDataQualityMetricsQueryVisitor)
    }
}
