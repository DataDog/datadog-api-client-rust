// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamPermissionSettingValue {
    #[serde(rename = "admins")]
    ADMINS,
    #[serde(rename = "members")]
    MEMBERS,
    #[serde(rename = "organization")]
    ORGANIZATION,
    #[serde(rename = "user_access_manage")]
    USER_ACCESS_MANAGE,
    #[serde(rename = "teams_manage")]
    TEAMS_MANAGE,
}

impl ToString for TeamPermissionSettingValue {
    fn to_string(&self) -> String {
        match self {
            Self::ADMINS => String::from("admins"),
            Self::MEMBERS => String::from("members"),
            Self::ORGANIZATION => String::from("organization"),
            Self::USER_ACCESS_MANAGE => String::from("user_access_manage"),
            Self::TEAMS_MANAGE => String::from("teams_manage"),
        }
    }
}
