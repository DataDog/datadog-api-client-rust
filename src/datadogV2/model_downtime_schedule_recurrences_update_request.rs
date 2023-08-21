// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleRecurrencesUpdateRequest {
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences", skip_serializing_if = "Option::is_none")]
    pub recurrences: Vec<DowntimeScheduleRecurrenceCreateUpdateRequest>,
    /// The timezone in which to schedule the downtime.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
}

