// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleRecurrencesResponse {
    /// The most recent actual start and end dates for a recurring downtime. For a canceled downtime,
this is the previously occurring downtime. For active downtimes, this is the ongoing downtime, and for scheduled
downtimes it is the upcoming downtime.
    #[serde(rename = "current_downtime", skip_serializing_if = "Option::is_none")]
    pub current_downtime: DowntimeScheduleCurrentDowntimeResponse,
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences", skip_serializing_if = "Option::is_none")]
    pub recurrences: Vec<DowntimeScheduleRecurrenceResponse>,
    /// The timezone in which to schedule the downtime. This affects recurring start and end dates.
Must match `display_timezone`.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
}

