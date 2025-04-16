// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Represents an escalation target, which can be a team, user, or schedule.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EscalationTarget {
    TeamTarget(Box<crate::datadogV2::model::TeamTarget>),
    UserTarget(Box<crate::datadogV2::model::UserTarget>),
    ScheduleTarget(Box<crate::datadogV2::model::ScheduleTarget>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EscalationTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::TeamTarget>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationTarget::TeamTarget(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::UserTarget>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationTarget::UserTarget(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ScheduleTarget>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationTarget::ScheduleTarget(_v));
            }
        }

        return Ok(EscalationTarget::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
