// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Any additional resources related to this schedule, such as teams and layers.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScheduleDataIncludedItem {
    TeamReference(Box<crate::datadogV2::model::TeamReference>),
    Layer(Box<crate::datadogV2::model::Layer>),
    ScheduleMember(Box<crate::datadogV2::model::ScheduleMember>),
    ScheduleUser(Box<crate::datadogV2::model::ScheduleUser>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScheduleDataIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::TeamReference>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleDataIncludedItem::TeamReference(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::Layer>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleDataIncludedItem::Layer(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ScheduleMember>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleDataIncludedItem::ScheduleMember(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ScheduleUser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScheduleDataIncludedItem::ScheduleUser(_v));
            }
        }

        return Ok(ScheduleDataIncludedItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
