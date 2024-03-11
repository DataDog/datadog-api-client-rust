// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Schedule for the downtime.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DowntimeScheduleUpdateRequest {
    DowntimeScheduleRecurrencesUpdateRequest(
        Box<crate::datadogV2::model::DowntimeScheduleRecurrencesUpdateRequest>,
    ),
    DowntimeScheduleOneTimeCreateUpdateRequest(
        Box<crate::datadogV2::model::DowntimeScheduleOneTimeCreateUpdateRequest>,
    ),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for DowntimeScheduleUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeScheduleRecurrencesUpdateRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    DowntimeScheduleUpdateRequest::DowntimeScheduleRecurrencesUpdateRequest(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DowntimeScheduleOneTimeCreateUpdateRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    DowntimeScheduleUpdateRequest::DowntimeScheduleOneTimeCreateUpdateRequest(_v),
                );
            }
        }

        return Ok(DowntimeScheduleUpdateRequest::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
