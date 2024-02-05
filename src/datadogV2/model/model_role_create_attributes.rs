// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the created role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCreateAttributes {
    /// Creation time of the role.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Time of last role modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Name of the role.
    #[serde(rename = "name")]
    pub name: String,
}

impl RoleCreateAttributes {
    pub fn new(name: String) -> RoleCreateAttributes {
        RoleCreateAttributes {
            created_at: None,
            modified_at: None,
            name,
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
}
