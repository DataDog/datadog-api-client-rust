// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamsField {
    #[serde(rename = "id")]
    ID,
    #[serde(rename = "name")]
    NAME,
    #[serde(rename = "handle")]
    HANDLE,
    #[serde(rename = "summary")]
    SUMMARY,
    #[serde(rename = "description")]
    DESCRIPTION,
    #[serde(rename = "avatar")]
    AVATAR,
    #[serde(rename = "banner")]
    BANNER,
    #[serde(rename = "visible_modules")]
    VISIBLE_MODULES,
    #[serde(rename = "hidden_modules")]
    HIDDEN_MODULES,
    #[serde(rename = "created_at")]
    CREATED_AT,
    #[serde(rename = "modified_at")]
    MODIFIED_AT,
    #[serde(rename = "user_count")]
    USER_COUNT,
    #[serde(rename = "link_count")]
    LINK_COUNT,
    #[serde(rename = "team_links")]
    TEAM_LINKS,
    #[serde(rename = "user_team_permissions")]
    USER_TEAM_PERMISSIONS,
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
        }
    }
}
