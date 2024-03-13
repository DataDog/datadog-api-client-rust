// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamsField {
    ID,
    NAME,
    HANDLE,
    SUMMARY,
    DESCRIPTION,
    AVATAR,
    BANNER,
    VISIBLE_MODULES,
    HIDDEN_MODULES,
    CREATED_AT,
    MODIFIED_AT,
    USER_COUNT,
    LINK_COUNT,
    TEAM_LINKS,
    USER_TEAM_PERMISSIONS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TeamsField {
    fn to_string(&self) -> String {
        match self {
            Self::ID => String::from("id"),
            Self::NAME => String::from("name"),
            Self::HANDLE => String::from("handle"),
            Self::SUMMARY => String::from("summary"),
            Self::DESCRIPTION => String::from("description"),
            Self::AVATAR => String::from("avatar"),
            Self::BANNER => String::from("banner"),
            Self::VISIBLE_MODULES => String::from("visible_modules"),
            Self::HIDDEN_MODULES => String::from("hidden_modules"),
            Self::CREATED_AT => String::from("created_at"),
            Self::MODIFIED_AT => String::from("modified_at"),
            Self::USER_COUNT => String::from("user_count"),
            Self::LINK_COUNT => String::from("link_count"),
            Self::TEAM_LINKS => String::from("team_links"),
            Self::USER_TEAM_PERMISSIONS => String::from("user_team_permissions"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TeamsField {
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

impl<'de> Deserialize<'de> for TeamsField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "id" => Self::ID,
            "name" => Self::NAME,
            "handle" => Self::HANDLE,
            "summary" => Self::SUMMARY,
            "description" => Self::DESCRIPTION,
            "avatar" => Self::AVATAR,
            "banner" => Self::BANNER,
            "visible_modules" => Self::VISIBLE_MODULES,
            "hidden_modules" => Self::HIDDEN_MODULES,
            "created_at" => Self::CREATED_AT,
            "modified_at" => Self::MODIFIED_AT,
            "user_count" => Self::USER_COUNT,
            "link_count" => Self::LINK_COUNT,
            "team_links" => Self::TEAM_LINKS,
            "user_team_permissions" => Self::USER_TEAM_PERMISSIONS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
