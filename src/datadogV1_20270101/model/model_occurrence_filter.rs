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
pub struct OccurrenceFilter {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1_20270101::model::OccurrenceFilterOperator,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: crate::datadogV1_20270101::model::OccurrenceFilterValue,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl OccurrenceFilter {
    pub fn new(operator: crate::datadogV1_20270101::model::OccurrenceFilterOperator, value: crate::datadogV1_20270101::model::OccurrenceFilterValue) -> OccurrenceFilter {
        OccurrenceFilter {
            operator,
            value,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for OccurrenceFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OccurrenceFilterVisitor;
        impl<'a> Visitor<'a> for OccurrenceFilterVisitor {
            type Value = OccurrenceFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<crate::datadogV1_20270101::model::OccurrenceFilterOperator> = None;
                let mut value: Option<crate::datadogV1_20270101::model::OccurrenceFilterValue> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV1_20270101::model::OccurrenceFilterOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV1_20270101::model::OccurrenceFilterValue::UnparsedObject(_value) => {
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
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = OccurrenceFilter {
                    operator,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OccurrenceFilterVisitor)
    }
}
