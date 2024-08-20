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
pub struct DowntimeRecurrence {
    /// How often to repeat as an integer.
    /// For example, to repeat every 3 days, select a type of `days` and a period of `3`.
    #[serde(rename = "period")]
    pub period: Option<i32>,
    /// The `RRULE` standard for defining recurring events (**requires to set "type" to rrule**)
    /// For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`.
    /// Most common `rrule` options from the [iCalendar Spec](<https://tools.ietf.org/html/rfc5545>) are supported.
    ///
    /// **Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`).
    /// More examples available in this [downtime guide](<https://docs.datadoghq.com/monitors/guide/suppress-alert-with-downtimes/?tab=api>)
    #[serde(rename = "rrule")]
    pub rrule: Option<String>,
    /// The type of recurrence. Choose from `days`, `weeks`, `months`, `years`, `rrule`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The date at which the recurrence should end as a POSIX timestamp.
    /// `until_occurences` and `until_date` are mutually exclusive.
    #[serde(
        rename = "until_date",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub until_date: Option<Option<i64>>,
    /// How many times the downtime is rescheduled.
    /// `until_occurences` and `until_date` are mutually exclusive.
    #[serde(
        rename = "until_occurrences",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub until_occurrences: Option<Option<i32>>,
    /// A list of week days to repeat on. Choose from `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` or `Sun`.
    /// Only applicable when type is weeks. First letter must be capitalized.
    #[serde(
        rename = "week_days",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub week_days: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeRecurrence {
    pub fn new() -> DowntimeRecurrence {
        DowntimeRecurrence {
            period: None,
            rrule: None,
            type_: None,
            until_date: None,
            until_occurrences: None,
            week_days: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn period(mut self, value: i32) -> Self {
        self.period = Some(value);
        self
    }

    pub fn rrule(mut self, value: String) -> Self {
        self.rrule = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn until_date(mut self, value: Option<i64>) -> Self {
        self.until_date = Some(value);
        self
    }

    pub fn until_occurrences(mut self, value: Option<i32>) -> Self {
        self.until_occurrences = Some(value);
        self
    }

    pub fn week_days(mut self, value: Option<Vec<String>>) -> Self {
        self.week_days = Some(value);
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

impl Default for DowntimeRecurrence {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeRecurrence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeRecurrenceVisitor;
        impl<'a> Visitor<'a> for DowntimeRecurrenceVisitor {
            type Value = DowntimeRecurrence;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut period: Option<i32> = None;
                let mut rrule: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut until_date: Option<Option<i64>> = None;
                let mut until_occurrences: Option<Option<i32>> = None;
                let mut week_days: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "period" => {
                            if v.is_null() {
                                continue;
                            }
                            period = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            if v.is_null() {
                                continue;
                            }
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "until_date" => {
                            until_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "until_occurrences" => {
                            until_occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "week_days" => {
                            week_days = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DowntimeRecurrence {
                    period,
                    rrule,
                    type_,
                    until_date,
                    until_occurrences,
                    week_days,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeRecurrenceVisitor)
    }
}
