// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object defining the recurrence of the downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleRecurrenceCreateUpdateRequest {
    /// The length of the downtime. Must begin with an integer and end with one of 'm', 'h', d', or 'w'.
    #[serde(rename = "duration")]
    pub duration: String,
    /// The `RRULE` standard for defining recurring events.
    /// For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`.
    /// Most common `rrule` options from the [iCalendar Spec](<https://tools.ietf.org/html/rfc5545>) are supported.
    ///
    /// **Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`).
    /// More examples available in this [downtime guide](<https://docs.datadoghq.com/monitors/guide/suppress-alert-with-downtimes/?tab=api>).
    #[serde(rename = "rrule")]
    pub rrule: String,
    /// ISO-8601 Datetime to start the downtime. Must not include a UTC offset. If not provided, the
    /// downtime starts the moment it is created.
    #[serde(rename = "start", default, with = "::serde_with::rust::double_option")]
    pub start: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleRecurrenceCreateUpdateRequest {
    pub fn new(duration: String, rrule: String) -> DowntimeScheduleRecurrenceCreateUpdateRequest {
        DowntimeScheduleRecurrenceCreateUpdateRequest {
            duration,
            rrule,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn start(&mut self, value: Option<String>) -> &mut Self {
        self.start = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleRecurrenceCreateUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleRecurrenceCreateUpdateRequestVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleRecurrenceCreateUpdateRequestVisitor {
            type Value = DowntimeScheduleRecurrenceCreateUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<String> = None;
                let mut rrule: Option<String> = None;
                let mut start: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;
                let rrule = rrule.ok_or_else(|| M::Error::missing_field("rrule"))?;

                let content = DowntimeScheduleRecurrenceCreateUpdateRequest {
                    duration,
                    rrule,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleRecurrenceCreateUpdateRequestVisitor)
    }
}
