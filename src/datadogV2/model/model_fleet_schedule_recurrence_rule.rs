// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the recurrence pattern for the schedule. Specifies when deployments should be
/// automatically triggered based on maintenance windows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetScheduleRecurrenceRule {
    /// List of days of the week when the schedule should trigger. Valid values are:
    /// "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun".
    #[serde(rename = "days_of_week")]
    pub days_of_week: Vec<String>,
    /// Duration of the maintenance window in minutes.
    #[serde(rename = "maintenance_window_duration")]
    pub maintenance_window_duration: i64,
    /// Start time of the maintenance window in 24-hour clock format (HH:MM).
    /// Deployments will be triggered at this time on the specified days.
    #[serde(rename = "start_maintenance_window")]
    pub start_maintenance_window: String,
    /// Timezone for the schedule in IANA Time Zone Database format (e.g., "America/New_York", "UTC").
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetScheduleRecurrenceRule {
    pub fn new(
        days_of_week: Vec<String>,
        maintenance_window_duration: i64,
        start_maintenance_window: String,
        timezone: String,
    ) -> FleetScheduleRecurrenceRule {
        FleetScheduleRecurrenceRule {
            days_of_week,
            maintenance_window_duration,
            start_maintenance_window,
            timezone,
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

impl<'de> Deserialize<'de> for FleetScheduleRecurrenceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetScheduleRecurrenceRuleVisitor;
        impl<'a> Visitor<'a> for FleetScheduleRecurrenceRuleVisitor {
            type Value = FleetScheduleRecurrenceRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut days_of_week: Option<Vec<String>> = None;
                let mut maintenance_window_duration: Option<i64> = None;
                let mut start_maintenance_window: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "days_of_week" => {
                            days_of_week =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maintenance_window_duration" => {
                            maintenance_window_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_maintenance_window" => {
                            start_maintenance_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let days_of_week =
                    days_of_week.ok_or_else(|| M::Error::missing_field("days_of_week"))?;
                let maintenance_window_duration = maintenance_window_duration
                    .ok_or_else(|| M::Error::missing_field("maintenance_window_duration"))?;
                let start_maintenance_window = start_maintenance_window
                    .ok_or_else(|| M::Error::missing_field("start_maintenance_window"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;

                let content = FleetScheduleRecurrenceRule {
                    days_of_week,
                    maintenance_window_duration,
                    start_maintenance_window,
                    timezone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetScheduleRecurrenceRuleVisitor)
    }
}
