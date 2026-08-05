// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the recurrence pattern for the schedule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetScheduleV2RecurrenceRule {
    /// Days of the week when the schedule triggers. Valid values are
    /// "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun".
    #[serde(rename = "days_of_week")]
    pub days_of_week: Option<Vec<String>>,
    /// Interval between schedule runs in weeks. 1 means the schedule runs every week
    /// on the specified days. Higher values repeat every N weeks.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Duration of the maintenance window in minutes.
    #[serde(rename = "maintenance_window_duration")]
    pub maintenance_window_duration: Option<i64>,
    /// Start time of the maintenance window in 24-hour clock format (HHMM).
    /// Deployments are triggered at this time on the specified days.
    #[serde(rename = "start_maintenance_window")]
    pub start_maintenance_window: Option<String>,
    /// Timezone in IANA Time Zone Database format.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetScheduleV2RecurrenceRule {
    pub fn new() -> FleetScheduleV2RecurrenceRule {
        FleetScheduleV2RecurrenceRule {
            days_of_week: None,
            interval: None,
            maintenance_window_duration: None,
            start_maintenance_window: None,
            timezone: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn days_of_week(mut self, value: Vec<String>) -> Self {
        self.days_of_week = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn maintenance_window_duration(mut self, value: i64) -> Self {
        self.maintenance_window_duration = Some(value);
        self
    }

    pub fn start_maintenance_window(mut self, value: String) -> Self {
        self.start_maintenance_window = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
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

impl Default for FleetScheduleV2RecurrenceRule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetScheduleV2RecurrenceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetScheduleV2RecurrenceRuleVisitor;
        impl<'a> Visitor<'a> for FleetScheduleV2RecurrenceRuleVisitor {
            type Value = FleetScheduleV2RecurrenceRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut days_of_week: Option<Vec<String>> = None;
                let mut interval: Option<i64> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            days_of_week =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maintenance_window_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            maintenance_window_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_maintenance_window" => {
                            if v.is_null() {
                                continue;
                            }
                            start_maintenance_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetScheduleV2RecurrenceRule {
                    days_of_week,
                    interval,
                    maintenance_window_duration,
                    start_maintenance_window,
                    timezone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetScheduleV2RecurrenceRuleVisitor)
    }
}
