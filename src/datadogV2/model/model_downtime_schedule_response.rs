// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The schedule that defines when the monitor starts, stops, and recurs. There are two types of schedules:
/// one-time and recurring. Recurring schedules may have up to five RRULE-based recurrences. If no schedules are
/// provided, the downtime will begin immediately and never end.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DowntimeScheduleResponse {
    DowntimeScheduleRecurrencesResponse(
        Box<crate::datadogV2::model::DowntimeScheduleRecurrencesResponse>,
    ),
    DowntimeScheduleOneTimeResponse(Box<crate::datadogV2::model::DowntimeScheduleOneTimeResponse>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for DowntimeScheduleResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeScheduleRecurrencesResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DowntimeScheduleResponse::DowntimeScheduleRecurrencesResponse(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeScheduleOneTimeResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DowntimeScheduleResponse::DowntimeScheduleOneTimeResponse(
                    _v,
                ));
            }
        }

        return Ok(DowntimeScheduleResponse::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
