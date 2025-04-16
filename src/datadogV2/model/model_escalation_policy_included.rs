// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Represents included related resources when retrieving an escalation policy, such as teams, steps, or targets.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EscalationPolicyIncluded {
    TeamReference(Box<crate::datadogV2::model::TeamReference>),
    EscalationPolicyStep(Box<crate::datadogV2::model::EscalationPolicyStep>),
    UserTarget(Box<crate::datadogV2::model::UserTarget>),
    ScheduleTarget(Box<crate::datadogV2::model::ScheduleTarget>),
    TeamTarget(Box<crate::datadogV2::model::TeamTarget>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EscalationPolicyIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::TeamReference>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationPolicyIncluded::TeamReference(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::EscalationPolicyStep>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(EscalationPolicyIncluded::EscalationPolicyStep(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::UserTarget>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationPolicyIncluded::UserTarget(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ScheduleTarget>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationPolicyIncluded::ScheduleTarget(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::TeamTarget>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(EscalationPolicyIncluded::TeamTarget(_v));
            }
        }

        return Ok(EscalationPolicyIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
