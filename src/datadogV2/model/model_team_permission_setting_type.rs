// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamPermissionSettingType {
    #[serde(rename = "team_permission_settings")]
    TEAM_PERMISSION_SETTINGS,
}

impl ToString for TeamPermissionSettingType {
    fn to_string(&self) -> String {
        match self {
            Self::TEAM_PERMISSION_SETTINGS => String::from("team_permission_settings"),
        }
    }
}
