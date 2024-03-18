// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An RRULE-based recurring downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleRecurrenceResponse {
    /// The length of the downtime. Must begin with an integer and end with one of 'm', 'h', d', or 'w'.
    #[serde(rename = "duration")]
    pub duration: Option<String>,
    /// The `RRULE` standard for defining recurring events.
    /// For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`.
    /// Most common `rrule` options from the [iCalendar Spec](<https://tools.ietf.org/html/rfc5545>) are supported.
    ///
    /// **Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`).
    /// More examples available in this [downtime guide](<https://docs.datadoghq.com/monitors/guide/suppress-alert-with-downtimes/?tab=api>).
    #[serde(rename = "rrule")]
    pub rrule: Option<String>,
    /// ISO-8601 Datetime to start the downtime. Must not include a UTC offset. If not provided, the
    /// downtime starts the moment it is created.
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleRecurrenceResponse {
    pub fn new() -> DowntimeScheduleRecurrenceResponse {
        DowntimeScheduleRecurrenceResponse {
            duration: None,
            rrule: None,
            start: None,
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: String) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn rrule(mut self, value: String) -> Self {
        self.rrule = Some(value);
        self
    }

    pub fn start(mut self, value: String) -> Self {
        self.start = Some(value);
        self
    }
}

impl Default for DowntimeScheduleRecurrenceResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleRecurrenceResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleRecurrenceResponseVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleRecurrenceResponseVisitor {
            type Value = DowntimeScheduleRecurrenceResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<String> = None;
                let mut rrule: Option<String> = None;
                let mut start: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            if v.is_null() {
                                continue;
                            }
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeScheduleRecurrenceResponse {
                    duration,
                    rrule,
                    start,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleRecurrenceResponseVisitor)
    }
}
