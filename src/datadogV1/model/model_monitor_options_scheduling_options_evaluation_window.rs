// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration options for the evaluation window. If `hour_starts` is set, no other fields may be set. Otherwise, `day_starts` and `month_starts` must be set together.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorOptionsSchedulingOptionsEvaluationWindow {
    /// The time of the day at which a one day cumulative evaluation window starts. Must be defined in UTC time in `HH:mm` format.
    #[serde(rename = "day_starts")]
    pub day_starts: Option<String>,
    /// The minute of the hour at which a one hour cumulative evaluation window starts.
    #[serde(rename = "hour_starts")]
    pub hour_starts: Option<i32>,
    /// The day of the month at which a one month cumulative evaluation window starts.
    #[serde(rename = "month_starts")]
    pub month_starts: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorOptionsSchedulingOptionsEvaluationWindow {
    pub fn new() -> MonitorOptionsSchedulingOptionsEvaluationWindow {
        MonitorOptionsSchedulingOptionsEvaluationWindow {
            day_starts: None,
            hour_starts: None,
            month_starts: None,
            _unparsed: false,
        }
    }

    pub fn day_starts(mut self, value: String) -> Self {
        self.day_starts = Some(value);
        self
    }

    pub fn hour_starts(mut self, value: i32) -> Self {
        self.hour_starts = Some(value);
        self
    }

    pub fn month_starts(mut self, value: i32) -> Self {
        self.month_starts = Some(value);
        self
    }
}

impl Default for MonitorOptionsSchedulingOptionsEvaluationWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorOptionsSchedulingOptionsEvaluationWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorOptionsSchedulingOptionsEvaluationWindowVisitor;
        impl<'a> Visitor<'a> for MonitorOptionsSchedulingOptionsEvaluationWindowVisitor {
            type Value = MonitorOptionsSchedulingOptionsEvaluationWindow;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut day_starts: Option<String> = None;
                let mut hour_starts: Option<i32> = None;
                let mut month_starts: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "day_starts" => {
                            if v.is_null() {
                                continue;
                            }
                            day_starts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour_starts" => {
                            if v.is_null() {
                                continue;
                            }
                            hour_starts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "month_starts" => {
                            if v.is_null() {
                                continue;
                            }
                            month_starts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorOptionsSchedulingOptionsEvaluationWindow {
                    day_starts,
                    hour_starts,
                    month_starts,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorOptionsSchedulingOptionsEvaluationWindowVisitor)
    }
}
