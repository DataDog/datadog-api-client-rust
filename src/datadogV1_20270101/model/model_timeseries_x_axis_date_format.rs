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
pub struct TimeseriesXAxisDateFormat {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "day")]
    pub day: Option<crate::datadogV1_20270101::model::TimeseriesXAxisDayDateFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "month")]
    pub month: Option<crate::datadogV1_20270101::model::TimeseriesXAxisMonthDateFormat>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TimeseriesXAxisDateFormat {
    pub fn new() -> TimeseriesXAxisDateFormat {
        TimeseriesXAxisDateFormat {
            day: None,
            month: None,
            _unparsed: false,
        }
    }
    
    pub fn day(mut self, value: crate::datadogV1_20270101::model::TimeseriesXAxisDayDateFormat) -> Self {
        self.day = Some(value);
        self
    }
    
    pub fn month(mut self, value: crate::datadogV1_20270101::model::TimeseriesXAxisMonthDateFormat) -> Self {
        self.month = Some(value);
        self
    }
    
}


impl Default for TimeseriesXAxisDateFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimeseriesXAxisDateFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesXAxisDateFormatVisitor;
        impl<'a> Visitor<'a> for TimeseriesXAxisDateFormatVisitor {
            type Value = TimeseriesXAxisDateFormat;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut day: Option<crate::datadogV1_20270101::model::TimeseriesXAxisDayDateFormat> = None;
                let mut month: Option<crate::datadogV1_20270101::model::TimeseriesXAxisMonthDateFormat> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "day" => {
                            if v.is_null() {
                                continue;
                            }
                            day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _day) = day {
                                match _day {
                                    crate::datadogV1_20270101::model::TimeseriesXAxisDayDateFormat::UnparsedObject(_day) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "month" => {
                            if v.is_null() {
                                continue;
                            }
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _month) = month {
                                match _month {
                                    crate::datadogV1_20270101::model::TimeseriesXAxisMonthDateFormat::UnparsedObject(_month) => {
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

                let content = TimeseriesXAxisDateFormat {
                    day,
                    month,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesXAxisDateFormatVisitor)
    }
}
