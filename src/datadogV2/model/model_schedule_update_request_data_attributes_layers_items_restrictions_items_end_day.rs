// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay {
    fn to_string(&self) -> String {
        match self {
            Self::MONDAY => String::from("monday"),
            Self::TUESDAY => String::from("tuesday"),
            Self::WEDNESDAY => String::from("wednesday"),
            Self::THURSDAY => String::from("thursday"),
            Self::FRIDAY => String::from("friday"),
            Self::SATURDAY => String::from("saturday"),
            Self::SUNDAY => String::from("sunday"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de>
    for ScheduleUpdateRequestDataAttributesLayersItemsRestrictionsItemsEndDay
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "monday" => Self::MONDAY,
            "tuesday" => Self::TUESDAY,
            "wednesday" => Self::WEDNESDAY,
            "thursday" => Self::THURSDAY,
            "friday" => Self::FRIDAY,
            "saturday" => Self::SATURDAY,
            "sunday" => Self::SUNDAY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
