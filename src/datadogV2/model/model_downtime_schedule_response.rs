// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// The schedule that defines when the monitor starts, stops, and recurs. There are two types of schedules:
/// one-time and recurring. Recurring schedules may have up to five RRULE-based recurrences. If no schedules are
/// provided, the downtime will begin immediately and never end.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DowntimeScheduleResponse {
    DowntimeScheduleRecurrencesResponse(
        crate::datadogV2::model::DowntimeScheduleRecurrencesResponse,
    ),
    DowntimeScheduleOneTimeResponse(crate::datadogV2::model::DowntimeScheduleOneTimeResponse),
}
