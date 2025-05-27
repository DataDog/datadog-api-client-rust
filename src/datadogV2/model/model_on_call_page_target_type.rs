// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnCallPageTargetType {
    TEAM_ID,
    TEAM_HANDLE,
    USER_ID,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OnCallPageTargetType {
    fn to_string(&self) -> String {
        match self {
            Self::TEAM_ID => String::from("team_id"),
            Self::TEAM_HANDLE => String::from("team_handle"),
            Self::USER_ID => String::from("user_id"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OnCallPageTargetType {
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

impl<'de> Deserialize<'de> for OnCallPageTargetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "team_id" => Self::TEAM_ID,
            "team_handle" => Self::TEAM_HANDLE,
            "user_id" => Self::USER_ID,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
