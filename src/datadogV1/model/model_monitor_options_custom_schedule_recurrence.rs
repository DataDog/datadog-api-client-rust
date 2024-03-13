// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a recurrence set on the monitor options for custom schedule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorOptionsCustomScheduleRecurrence {
    /// Defines the recurrence rule (RRULE) for a given schedule.
    #[serde(rename = "rrule")]
    pub rrule: Option<String>,
    /// Defines the start date and time of the recurring schedule.
    #[serde(rename = "start")]
    pub start: Option<String>,
    /// Defines the timezone the schedule runs on.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorOptionsCustomScheduleRecurrence {
    pub fn new() -> MonitorOptionsCustomScheduleRecurrence {
        MonitorOptionsCustomScheduleRecurrence {
            rrule: None,
            start: None,
            timezone: None,
            _unparsed: false,
        }
    }

    pub fn rrule(mut self, value: String) -> Self {
        self.rrule = Some(value);
        self
    }

    pub fn start(mut self, value: String) -> Self {
        self.start = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl Default for MonitorOptionsCustomScheduleRecurrence {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorOptionsCustomScheduleRecurrence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorOptionsCustomScheduleRecurrenceVisitor;
        impl<'a> Visitor<'a> for MonitorOptionsCustomScheduleRecurrenceVisitor {
            type Value = MonitorOptionsCustomScheduleRecurrence;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rrule: Option<String> = None;
                let mut start: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorOptionsCustomScheduleRecurrence {
                    rrule,
                    start,
                    timezone,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorOptionsCustomScheduleRecurrenceVisitor)
    }
}
