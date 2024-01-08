// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to roles.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToRoles {
    /// An array containing type and the unique identifier of a role.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::RelationshipToRoleData>>,
}

impl RelationshipToRoles {
    pub fn new() -> RelationshipToRoles {
        RelationshipToRoles { data: None }
    }
}
impl Default for RelationshipToRoles {
    fn default() -> Self {
        Self::new()
    }
}
