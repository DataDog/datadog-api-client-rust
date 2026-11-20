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
pub struct DatasetCompute {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1_20270101::model::DatasetCompute20270101Aggregation,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "column")]
    pub column: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DatasetCompute {
    pub fn new(aggregation: crate::datadogV1_20270101::model::DatasetCompute20270101Aggregation, name: String) -> DatasetCompute {
        DatasetCompute {
            aggregation,
            column: None,
            interval: None,
            name,
            _unparsed: false,
        }
    }
    
    pub fn column(mut self, value: String) -> Self {
        self.column = Some(value);
        self
    }
    
    pub fn interval(mut self, value: f64) -> Self {
        self.interval = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for DatasetCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetComputeVisitor;
        impl<'a> Visitor<'a> for DatasetComputeVisitor {
            type Value = DatasetCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1_20270101::model::DatasetCompute20270101Aggregation> = None;
                let mut column: Option<String> = None;
                let mut interval: Option<f64> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1_20270101::model::DatasetCompute20270101Aggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "column" => {
                            if v.is_null() {
                                continue;
                            }
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "interval" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let aggregation = aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = DatasetCompute {
                    aggregation,
                    column,
                    interval,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetComputeVisitor)
    }
}
