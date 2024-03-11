// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamPermissionSettingValue {
    ADMINS,
    MEMBERS,
    ORGANIZATION,
    USER_ACCESS_MANAGE,
    TEAMS_MANAGE,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for TeamPermissionSettingValue {
    fn to_string(&self) -> String {
        match self {
            Self::ADMINS => String::from("admins"),
            Self::MEMBERS => String::from("members"),
            Self::ORGANIZATION => String::from("organization"),
            Self::USER_ACCESS_MANAGE => String::from("user_access_manage"),
            Self::TEAMS_MANAGE => String::from("teams_manage"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TeamPermissionSettingValue {
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

impl<'de> Deserialize<'de> for TeamPermissionSettingValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "admins" => Self::ADMINS,
            "members" => Self::MEMBERS,
            "organization" => Self::ORGANIZATION,
            "user_access_manage" => Self::USER_ACCESS_MANAGE,
            "teams_manage" => Self::TEAMS_MANAGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
