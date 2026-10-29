// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeatureFlagChangeEventAction {
    CREATED,
    UPDATED,
    DELETED,
    ARCHIVED,
    UNARCHIVED,
    SCHEDULED,
    STARTED,
    PAUSED,
    UNPAUSED,
    STOPPED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FeatureFlagChangeEventAction {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED => String::from("created"),
            Self::UPDATED => String::from("updated"),
            Self::DELETED => String::from("deleted"),
            Self::ARCHIVED => String::from("archived"),
            Self::UNARCHIVED => String::from("unarchived"),
            Self::SCHEDULED => String::from("scheduled"),
            Self::STARTED => String::from("started"),
            Self::PAUSED => String::from("paused"),
            Self::UNPAUSED => String::from("unpaused"),
            Self::STOPPED => String::from("stopped"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FeatureFlagChangeEventAction {
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

impl<'de> Deserialize<'de> for FeatureFlagChangeEventAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created" => Self::CREATED,
            "updated" => Self::UPDATED,
            "deleted" => Self::DELETED,
            "archived" => Self::ARCHIVED,
            "unarchived" => Self::UNARCHIVED,
            "scheduled" => Self::SCHEDULED,
            "started" => Self::STARTED,
            "paused" => Self::PAUSED,
            "unpaused" => Self::UNPAUSED,
            "stopped" => Self::STOPPED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
