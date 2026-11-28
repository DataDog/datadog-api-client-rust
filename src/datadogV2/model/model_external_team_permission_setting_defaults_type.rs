// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalTeamPermissionSettingDefaultsType {
    EXTERNAL_TEAM_PERMISSION_SETTING_DEFAULTS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExternalTeamPermissionSettingDefaultsType {
    fn to_string(&self) -> String {
        match self {
            Self::EXTERNAL_TEAM_PERMISSION_SETTING_DEFAULTS => {
                String::from("external_team_permission_setting_defaults")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExternalTeamPermissionSettingDefaultsType {
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

impl<'de> Deserialize<'de> for ExternalTeamPermissionSettingDefaultsType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "external_team_permission_setting_defaults" => {
                Self::EXTERNAL_TEAM_PERMISSION_SETTING_DEFAULTS
            }
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
