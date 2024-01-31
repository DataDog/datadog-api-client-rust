// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of a permission.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    pub fn with_created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn with_description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn with_display_name(&mut self, value: String) -> &mut Self {
        self.display_name = Some(value);
        self
    }

    pub fn with_display_type(&mut self, value: String) -> &mut Self {
        self.display_type = Some(value);
        self
    }

    pub fn with_group_name(&mut self, value: String) -> &mut Self {
        self.group_name = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_restricted(&mut self, value: bool) -> &mut Self {
        self.restricted = Some(value);
        self
    }
}
impl Default for PermissionAttributes {
    fn default() -> Self {
        Self::new()
    }
}
