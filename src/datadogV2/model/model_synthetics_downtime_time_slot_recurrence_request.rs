// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Recurrence settings for a Synthetics downtime time slot.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDowntimeTimeSlotRecurrenceRequest {
    /// A specific date and time used to define the start or end of a Synthetics downtime time slot.
    #[serde(rename = "end")]
    pub end: Option<crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate>,
    /// The recurrence frequency of a Synthetics downtime time slot.
    #[serde(rename = "frequency")]
    pub frequency: crate::datadogV2::model::SyntheticsDowntimeFrequency,
    /// The interval between recurrences, relative to the frequency.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Days of the week for a Synthetics downtime recurrence schedule.
    #[serde(rename = "weekdays")]
    pub weekdays: Option<Vec<crate::datadogV2::model::SyntheticsDowntimeWeekday>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDowntimeTimeSlotRecurrenceRequest {
    pub fn new(
        frequency: crate::datadogV2::model::SyntheticsDowntimeFrequency,
    ) -> SyntheticsDowntimeTimeSlotRecurrenceRequest {
        SyntheticsDowntimeTimeSlotRecurrenceRequest {
            end: None,
            frequency,
            interval: None,
            weekdays: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end(mut self, value: crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate) -> Self {
        self.end = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn weekdays(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsDowntimeWeekday>,
    ) -> Self {
        self.weekdays = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsDowntimeTimeSlotRecurrenceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDowntimeTimeSlotRecurrenceRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsDowntimeTimeSlotRecurrenceRequestVisitor {
            type Value = SyntheticsDowntimeTimeSlotRecurrenceRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<crate::datadogV2::model::SyntheticsDowntimeTimeSlotDate> = None;
                let mut frequency: Option<crate::datadogV2::model::SyntheticsDowntimeFrequency> =
                    None;
                let mut interval: Option<i64> = None;
                let mut weekdays: Option<Vec<crate::datadogV2::model::SyntheticsDowntimeWeekday>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "frequency" => {
                            frequency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _frequency) = frequency {
                                match _frequency {
                                    crate::datadogV2::model::SyntheticsDowntimeFrequency::UnparsedObject(_frequency) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "weekdays" => {
                            if v.is_null() {
                                continue;
                            }
                            weekdays = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let frequency = frequency.ok_or_else(|| M::Error::missing_field("frequency"))?;

                let content = SyntheticsDowntimeTimeSlotRecurrenceRequest {
                    end,
                    frequency,
                    interval,
                    weekdays,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDowntimeTimeSlotRecurrenceRequestVisitor)
    }
}
