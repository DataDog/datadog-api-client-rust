// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRecurrence {
    /// How often to repeat as an integer.
For example, to repeat every 3 days, select a type of `days` and a period of `3`.
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: i32,
    /// The `RRULE` standard for defining recurring events (**requires to set "type" to rrule**)
For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`.
Most common `rrule` options from the [iCalendar Spec](https://tools.ietf.org/html/rfc5545) are supported.

**Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`).
More examples available in this [downtime guide](https://docs.datadoghq.com/monitors/guide/suppress-alert-with-downtimes/?tab=api)
    #[serde(rename = "rrule", skip_serializing_if = "Option::is_none")]
    pub rrule: String,
    /// The type of recurrence. Choose from `days`, `weeks`, `months`, `years`, `rrule`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
    /// The date at which the recurrence should end as a POSIX timestamp.
`until_occurences` and `until_date` are mutually exclusive.
    #[serde(rename = "until_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub until_date: Option<Int64>,
    /// How many times the downtime is rescheduled.
`until_occurences` and `until_date` are mutually exclusive.
    #[serde(rename = "until_occurrences", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub until_occurrences: Option<Int32>,
    /// A list of week days to repeat on. Choose from `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` or `Sun`.
Only applicable when type is weeks. First letter must be capitalized.
    #[serde(rename = "week_days", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub week_days: datadog.NullableList[String],
}

