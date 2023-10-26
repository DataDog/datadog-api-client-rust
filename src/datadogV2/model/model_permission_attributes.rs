// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PermissionAttributes {
    /// Creation time of the permission.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Description of the permission.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Displayed name for the permission.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Display type.
    #[serde(rename = "display_type", skip_serializing_if = "Option::is_none")]
    pub display_type: Option<String>,
    /// Name of the permission group.
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Name of the permission.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not the permission is restricted.
    #[serde(rename = "restricted", skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
}

impl PermissionAttributes {
    /// Attributes of a permission.
    pub fn new() -> PermissionAttributes {
        PermissionAttributes {
            created: None,
            description: None,
            display_name: None,
            display_type: None,
            group_name: None,
            name: None,
            restricted: None,
        }
    }
}
