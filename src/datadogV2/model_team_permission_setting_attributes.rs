// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamPermissionSettingAttributes {
    /// The identifier for the action
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: TeamPermissionSettingSerializerAction,
    /// Whether or not the permission setting is editable by the current user
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: bool,
    /// Possible values for action
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Vec<TeamPermissionSettingValue>,
    /// The team permission name
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// What type of user is allowed to perform the specified action
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: TeamPermissionSettingValue,
}

