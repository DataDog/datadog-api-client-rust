// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PermissionAttributes {
    /// Creation time of the permission.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Description of the permission.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Displayed name for the permission.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Display type.
    #[serde(rename = "display_type")]
    pub display_type: Option<String>,
    /// Name of the permission group.
    #[serde(rename = "group_name")]
    pub group_name: Option<String>,
    /// Name of the permission.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether or not the permission is restricted.
    #[serde(rename = "restricted")]
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
