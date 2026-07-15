// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Represents a union of related resources included in the response, such as responder groups, shifts, schedules, and users.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScheduleOnCallRespondersIncluded {
    ScheduleOnCallResponderData(Box<crate::datadogV2::model::ScheduleOnCallResponderData>),
    ShiftData(Box<crate::datadogV2::model::ShiftData>),
    ScheduleData(Box<crate::datadogV2::model::ScheduleData>),
    User(Box<crate::datadogV2::model::User>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScheduleOnCallRespondersIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ScheduleOnCallResponderData>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleOnCallRespondersIncluded::ScheduleOnCallResponderData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ShiftData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleOnCallRespondersIncluded::ShiftData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ScheduleData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleOnCallRespondersIncluded::ScheduleData(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::User>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleOnCallRespondersIncluded::User(_v));
            }
        }

        return Ok(ScheduleOnCallRespondersIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
