// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptionsSchedulingOptionsEvaluationWindow {
    /// The time of the day at which a one day cumulative evaluation window starts. Must be defined in UTC time in `HH:mm` format.
    #[serde(rename = "day_starts", skip_serializing_if = "Option::is_none")]
    pub day_starts: String,
    /// The minute of the hour at which a one hour cumulative evaluation window starts.
    #[serde(rename = "hour_starts", skip_serializing_if = "Option::is_none")]
    pub hour_starts: i32,
    /// The day of the month at which a one month cumulative evaluation window starts.
    #[serde(rename = "month_starts", skip_serializing_if = "Option::is_none")]
    pub month_starts: i32,
}

