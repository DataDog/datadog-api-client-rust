// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration options for the evaluation window. If `hour_starts` is set, no other fields may be set. Otherwise, `day_starts` and `month_starts` must be set together.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}

impl MonitorOptionsSchedulingOptionsEvaluationWindow {
    pub fn new() -> MonitorOptionsSchedulingOptionsEvaluationWindow {
        MonitorOptionsSchedulingOptionsEvaluationWindow {
            day_starts: None,
            hour_starts: None,
            month_starts: None,
        }
    }

    pub fn with_day_starts(&mut self, value: String) -> &mut Self {
        self.day_starts = Some(value);
        self
    }

    pub fn with_hour_starts(&mut self, value: i32) -> &mut Self {
        self.hour_starts = Some(value);
        self
    }

    pub fn with_month_starts(&mut self, value: i32) -> &mut Self {
        self.month_starts = Some(value);
        self
    }
}
impl Default for MonitorOptionsSchedulingOptionsEvaluationWindow {
    fn default() -> Self {
        Self::new()
    }
}
