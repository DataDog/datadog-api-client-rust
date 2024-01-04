// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// User team permission attributes
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamPermissionAttributes {
    /// Object of team permission actions and boolean values that a logged in user can perform on this team.
    #[serde(rename = "permissions")]
    pub permissions: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl UserTeamPermissionAttributes {
    pub fn new() -> UserTeamPermissionAttributes {
        UserTeamPermissionAttributes { permissions: None }
    }
}
