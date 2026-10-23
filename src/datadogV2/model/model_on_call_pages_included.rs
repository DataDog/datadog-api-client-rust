// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A resource related to a page.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OnCallPagesIncluded {
    ScheduleUser(Box<crate::datadogV2::model::ScheduleUser>),
    TeamReference(Box<crate::datadogV2::model::TeamReference>),
    OnCallPageScheduledAction(Box<crate::datadogV2::model::OnCallPageScheduledAction>),
    OnCallPageLink(Box<crate::datadogV2::model::OnCallPageLink>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for OnCallPagesIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ScheduleUser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(OnCallPagesIncluded::ScheduleUser(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::TeamReference>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(OnCallPagesIncluded::TeamReference(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::OnCallPageScheduledAction>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(OnCallPagesIncluded::OnCallPageScheduledAction(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::OnCallPageLink>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(OnCallPagesIncluded::OnCallPageLink(_v));
            }
        }

        return Ok(OnCallPagesIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
