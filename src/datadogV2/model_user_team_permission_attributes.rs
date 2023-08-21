// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamPermissionAttributes {
    /// Object of team permission actions and boolean values indicating of the currently logged in user can perform them on this team
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: interface{},
}

