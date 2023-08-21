// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamPermission {
    /// User team permission attributes
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: UserTeamPermissionAttributes,
    /// The user team permission's identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// User team permission type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: UserTeamPermissionType,
}

