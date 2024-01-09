// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object defining the recurrence of the downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleRecurrenceCreateUpdateRequest {
    /// The length of the downtime. Must begin with an integer and end with one of 'm', 'h', d', or 'w'.
    #[serde(rename = "duration")]
    pub duration: String,
    /// The `RRULE` standard for defining recurring events.
    /// For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`.
    /// Most common `rrule` options from the [iCalendar Spec](https://tools.ietf.org/html/rfc5545) are supported.
    ///
    /// **Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`).
    /// More examples available in this [downtime guide](https://docs.datadoghq.com/monitors/guide/suppress-alert-with-downtimes/?tab=api).
    #[serde(rename = "rrule")]
    pub rrule: String,
    /// ISO-8601 Datetime to start the downtime. Must not include a UTC offset. If not provided, the
    /// downtime starts the moment it is created.
    #[serde(rename = "start", default, with = "::serde_with::rust::double_option")]
    pub start: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}

impl DowntimeScheduleRecurrenceCreateUpdateRequest {
    pub fn new(duration: String, rrule: String) -> DowntimeScheduleRecurrenceCreateUpdateRequest {
        DowntimeScheduleRecurrenceCreateUpdateRequest {
            duration,
            rrule,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
        }
    }
}
