// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Time interval for return criteria.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionReturnCriteriaTimeInterval {
    /// Type of time interval for return criteria.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalType,
    /// Unit of time for retention return criteria interval.
    #[serde(rename = "unit")]
    pub unit: crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalUnit,
    /// Value of the time interval.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionReturnCriteriaTimeInterval {
    pub fn new(
        type_: crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalType,
        unit: crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalUnit,
        value: f64,
    ) -> RetentionReturnCriteriaTimeInterval {
        RetentionReturnCriteriaTimeInterval {
            type_,
            unit,
            value,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RetentionReturnCriteriaTimeInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionReturnCriteriaTimeIntervalVisitor;
        impl<'a> Visitor<'a> for RetentionReturnCriteriaTimeIntervalVisitor {
            type Value = RetentionReturnCriteriaTimeInterval;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<
                    crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalType,
                > = None;
                let mut unit: Option<
                    crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalUnit,
                > = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unit) = unit {
                                match _unit {
                                    crate::datadogV1::model::RetentionReturnCriteriaTimeIntervalUnit::UnparsedObject(_unit) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = RetentionReturnCriteriaTimeInterval {
                    type_,
                    unit,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionReturnCriteriaTimeIntervalVisitor)
    }
}
