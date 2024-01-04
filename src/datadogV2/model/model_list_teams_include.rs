// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListTeamsInclude {
    #[serde(rename = "team_links")]
    TEAM_LINKS,
    #[serde(rename = "user_team_permissions")]
    USER_TEAM_PERMISSIONS,
}

impl ToString for ListTeamsInclude {
    fn to_string(&self) -> String {
        match self {
            Self::TEAM_LINKS => String::from("team_links"),
            Self::USER_TEAM_PERMISSIONS => String::from("user_team_permissions"),
        }
    }
}
