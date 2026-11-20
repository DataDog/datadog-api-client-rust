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
pub struct CalendarShift {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "unit")]
    pub unit: crate::datadogV1_20270101::model::CalendarShiftUnit,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CalendarShift {
    pub fn new(amount: f64, unit: crate::datadogV1_20270101::model::CalendarShiftUnit) -> CalendarShift {
        CalendarShift {
            amount,
            unit,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for CalendarShift {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CalendarShiftVisitor;
        impl<'a> Visitor<'a> for CalendarShiftVisitor {
            type Value = CalendarShift;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut amount: Option<f64> = None;
                let mut unit: Option<crate::datadogV1_20270101::model::CalendarShiftUnit> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "amount" => {
                            amount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unit) = unit {
                                match _unit {
                                    crate::datadogV1_20270101::model::CalendarShiftUnit::UnparsedObject(_unit) => {
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
                let amount = amount.ok_or_else(|| M::Error::missing_field("amount"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;

                let content = CalendarShift {
                    amount,
                    unit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CalendarShiftVisitor)
    }
}
