// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A recurring downtime schedule definition.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleRecurrencesResponse {
    /// The most recent actual start and end dates for a recurring downtime. For a canceled downtime,
    /// this is the previously occurring downtime. For active downtimes, this is the ongoing downtime, and for scheduled
    /// downtimes it is the upcoming downtime.
    #[serde(rename = "current_downtime")]
    pub current_downtime: Option<crate::datadogV2::model::DowntimeScheduleCurrentDowntimeResponse>,
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceResponse>,
    /// The timezone in which to schedule the downtime. This affects recurring start and end dates.
    /// Must match `display_timezone`.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

impl DowntimeScheduleRecurrencesResponse {
    pub fn new(
        recurrences: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceResponse>,
    ) -> DowntimeScheduleRecurrencesResponse {
        DowntimeScheduleRecurrencesResponse {
            current_downtime: None,
            recurrences,
            timezone: None,
        }
    }

    pub fn with_current_downtime(
        &mut self,
        value: crate::datadogV2::model::DowntimeScheduleCurrentDowntimeResponse,
    ) -> &mut Self {
        self.current_downtime = Some(value);
        self
    }

    pub fn with_timezone(&mut self, value: String) -> &mut Self {
        self.timezone = Some(value);
        self
    }
}
