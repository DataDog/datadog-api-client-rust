// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserTeamPermissionType {
    #[serde(rename = "user_team_permissions")]
    USER_TEAM_PERMISSIONS,
}
impl ToString for UserTeamPermissionType {
    fn to_string(&self) -> String {
        match self {
            Self::USER_TEAM_PERMISSIONS => String::from("user_team_permissions"),
        }
    }
}
