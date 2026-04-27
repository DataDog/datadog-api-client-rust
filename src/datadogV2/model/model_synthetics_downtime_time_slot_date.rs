// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A specific date and time used to define the start or end of a Synthetics downtime time slot.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDowntimeTimeSlotDate {
    /// The day component of the date (1-31).
    #[serde(rename = "day")]
    pub day: i64,
    /// The hour component of the time (0-23).
    #[serde(rename = "hour")]
    pub hour: i64,
    /// The minute component of the time (0-59).
    #[serde(rename = "minute")]
    pub minute: i64,
    /// The month component of the date (1-12).
    #[serde(rename = "month")]
    pub month: i64,
    /// The year component of the date.
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDowntimeTimeSlotDate {
    pub fn new(
        day: i64,
        hour: i64,
        minute: i64,
        month: i64,
        year: i64,
    ) -> SyntheticsDowntimeTimeSlotDate {
        SyntheticsDowntimeTimeSlotDate {
            day,
            hour,
            minute,
            month,
            year,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsDowntimeTimeSlotDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDowntimeTimeSlotDateVisitor;
        impl<'a> Visitor<'a> for SyntheticsDowntimeTimeSlotDateVisitor {
            type Value = SyntheticsDowntimeTimeSlotDate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut day: Option<i64> = None;
                let mut hour: Option<i64> = None;
                let mut minute: Option<i64> = None;
                let mut month: Option<i64> = None;
                let mut year: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "day" => {
                            day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minute" => {
                            minute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "month" => {
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "year" => {
                            year = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let day = day.ok_or_else(|| M::Error::missing_field("day"))?;
                let hour = hour.ok_or_else(|| M::Error::missing_field("hour"))?;
                let minute = minute.ok_or_else(|| M::Error::missing_field("minute"))?;
                let month = month.ok_or_else(|| M::Error::missing_field("month"))?;
                let year = year.ok_or_else(|| M::Error::missing_field("year"))?;

                let content = SyntheticsDowntimeTimeSlotDate {
                    day,
                    hour,
                    minute,
                    month,
                    year,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDowntimeTimeSlotDateVisitor)
    }
}
