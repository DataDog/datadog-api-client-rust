// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Calendar interval definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CalendarInterval {
    /// Alignment of the interval.
    #[serde(rename = "alignment")]
    pub alignment: Option<String>,
    /// Quantity of the interval.
    #[serde(rename = "quantity")]
    pub quantity: Option<i64>,
    /// Timezone for the interval.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Type of calendar interval.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::CalendarIntervalType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CalendarInterval {
    pub fn new(type_: crate::datadogV1::model::CalendarIntervalType) -> CalendarInterval {
        CalendarInterval {
            alignment: None,
            quantity: None,
            timezone: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn alignment(mut self, value: String) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CalendarInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CalendarIntervalVisitor;
        impl<'a> Visitor<'a> for CalendarIntervalVisitor {
            type Value = CalendarInterval;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alignment: Option<String> = None;
                let mut quantity: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::CalendarIntervalType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alignment" => {
                            if v.is_null() {
                                continue;
                            }
                            alignment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quantity" => {
                            if v.is_null() {
                                continue;
                            }
                            quantity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::CalendarIntervalType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CalendarInterval {
                    alignment,
                    quantity,
                    timezone,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CalendarIntervalVisitor)
    }
}
