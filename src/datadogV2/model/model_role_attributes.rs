// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAttributes {
    /// Creation time of the role.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Time of last role modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// The name of the role. The name is neither unique nor a stable identifier of the role.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Number of users with that role.
    #[serde(rename = "user_count")]
    pub user_count: Option<i64>,
}

impl RoleAttributes {
    pub fn new() -> RoleAttributes {
        RoleAttributes {
            created_at: None,
            modified_at: None,
            name: None,
            user_count: None,
        }
    }

    pub fn created_at(&mut self, value: String) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(&mut self, value: String) -> &mut Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn user_count(&mut self, value: i64) -> &mut Self {
        self.user_count = Some(value);
        self
    }
}

impl Default for RoleAttributes {
    fn default() -> Self {
        Self::new()
    }
}
