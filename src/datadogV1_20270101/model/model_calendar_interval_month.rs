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
pub struct CalendarIntervalMonth {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alignment")]
    pub alignment: Option<crate::datadogV1_20270101::model::DayOfTheMonth>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "quantity")]
    pub quantity: Option<f64>,
    /// IANA time zone identifier, for example America/New_York.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::CalendarIntervalMonthType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CalendarIntervalMonth {
    pub fn new(type_: crate::datadogV1_20270101::model::CalendarIntervalMonthType) -> CalendarIntervalMonth {
        CalendarIntervalMonth {
            alignment: None,
            quantity: None,
            timezone: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn alignment(mut self, value: crate::datadogV1_20270101::model::DayOfTheMonth) -> Self {
        self.alignment = Some(value);
        self
    }
    
    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }
    
    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for CalendarIntervalMonth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CalendarIntervalMonthVisitor;
        impl<'a> Visitor<'a> for CalendarIntervalMonthVisitor {
            type Value = CalendarIntervalMonth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alignment: Option<crate::datadogV1_20270101::model::DayOfTheMonth> = None;
                let mut quantity: Option<f64> = None;
                let mut timezone: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::CalendarIntervalMonthType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alignment" => {
                            if v.is_null() {
                                continue;
                            }
                            alignment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _alignment) = alignment {
                                match _alignment {
                                    crate::datadogV1_20270101::model::DayOfTheMonth::UnparsedObject(_alignment) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "quantity" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            quantity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::CalendarIntervalMonthType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CalendarIntervalMonth {
                    alignment,
                    quantity,
                    timezone,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CalendarIntervalMonthVisitor)
    }
}
