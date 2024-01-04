// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration for a recurrence set on the monitor options for custom schedule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}

impl MonitorOptionsCustomScheduleRecurrence {
    pub fn new() -> MonitorOptionsCustomScheduleRecurrence {
        MonitorOptionsCustomScheduleRecurrence {
            rrule: None,
            start: None,
            timezone: None,
        }
    }
}
