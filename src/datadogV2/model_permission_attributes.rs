// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionAttributes {
    /// Creation time of the permission.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// Description of the permission.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Displayed name for the permission.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Display type.
    #[serde(rename = "display_type", skip_serializing_if = "Option::is_none")]
    pub display_type: String,
    /// Name of the permission group.
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: String,
    /// Name of the permission.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Whether or not the permission is restricted.
    #[serde(rename = "restricted", skip_serializing_if = "Option::is_none")]
    pub restricted: bool,
}

