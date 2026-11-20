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
pub struct GraphQuerySearchFilter {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1_20270101::model::GraphQuerySearchFilterOperator,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV1_20270101::model::GraphTarget>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GraphQuerySearchFilter {
    pub fn new(name: String, operator: crate::datadogV1_20270101::model::GraphQuerySearchFilterOperator, value: f64) -> GraphQuerySearchFilter {
        GraphQuerySearchFilter {
            name,
            operator,
            target: None,
            value,
            _unparsed: false,
        }
    }
    
    pub fn target(mut self, value: crate::datadogV1_20270101::model::GraphTarget) -> Self {
        self.target = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GraphQuerySearchFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GraphQuerySearchFilterVisitor;
        impl<'a> Visitor<'a> for GraphQuerySearchFilterVisitor {
            type Value = GraphQuerySearchFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut operator: Option<crate::datadogV1_20270101::model::GraphQuerySearchFilterOperator> = None;
                let mut target: Option<crate::datadogV1_20270101::model::GraphTarget> = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV1_20270101::model::GraphQuerySearchFilterOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV1_20270101::model::GraphTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = GraphQuerySearchFilter {
                    name,
                    operator,
                    target,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GraphQuerySearchFilterVisitor)
    }
}
