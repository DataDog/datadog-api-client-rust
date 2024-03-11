// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListTeamsInclude {
    TEAM_LINKS,
    USER_TEAM_PERMISSIONS,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for ListTeamsInclude {
    fn to_string(&self) -> String {
        match self {
            Self::TEAM_LINKS => String::from("team_links"),
            Self::USER_TEAM_PERMISSIONS => String::from("user_team_permissions"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ListTeamsInclude {
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

impl<'de> Deserialize<'de> for ListTeamsInclude {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "team_links" => Self::TEAM_LINKS,
            "user_team_permissions" => Self::USER_TEAM_PERMISSIONS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
