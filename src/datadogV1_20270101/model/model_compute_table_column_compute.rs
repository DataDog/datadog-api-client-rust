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
pub struct ComputeTableColumnCompute {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "modifiers")]
    pub modifiers: Option<Vec<crate::datadogV1_20270101::model::TableColumnModifier>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "property")]
    pub property: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "scalar_aggregation")]
    pub scalar_aggregation: Option<crate::datadogV1_20270101::model::ScalarAggregator>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ComputeTableColumnCompute {
    pub fn new(aggregation: String, query: String) -> ComputeTableColumnCompute {
        ComputeTableColumnCompute {
            aggregation,
            filter: None,
            modifiers: None,
            property: None,
            query,
            scalar_aggregation: None,
            _unparsed: false,
        }
    }
    
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    
    pub fn modifiers(mut self, value: Vec<crate::datadogV1_20270101::model::TableColumnModifier>) -> Self {
        self.modifiers = Some(value);
        self
    }
    
    pub fn property(mut self, value: String) -> Self {
        self.property = Some(value);
        self
    }
    
    pub fn scalar_aggregation(mut self, value: crate::datadogV1_20270101::model::ScalarAggregator) -> Self {
        self.scalar_aggregation = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ComputeTableColumnCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ComputeTableColumnComputeVisitor;
        impl<'a> Visitor<'a> for ComputeTableColumnComputeVisitor {
            type Value = ComputeTableColumnCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<String> = None;
                let mut filter: Option<String> = None;
                let mut modifiers: Option<Vec<crate::datadogV1_20270101::model::TableColumnModifier>> = None;
                let mut property: Option<String> = None;
                let mut query: Option<String> = None;
                let mut scalar_aggregation: Option<crate::datadogV1_20270101::model::ScalarAggregator> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "modifiers" => {
                            if v.is_null() {
                                continue;
                            }
                            modifiers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "property" => {
                            if v.is_null() {
                                continue;
                            }
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "scalar_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            scalar_aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scalar_aggregation) = scalar_aggregation {
                                match _scalar_aggregation {
                                    crate::datadogV1_20270101::model::ScalarAggregator::UnparsedObject(_scalar_aggregation) => {
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
                let aggregation = aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = ComputeTableColumnCompute {
                    aggregation,
                    filter,
                    modifiers,
                    property,
                    query,
                    scalar_aggregation,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ComputeTableColumnComputeVisitor)
    }
}
