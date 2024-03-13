// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to role object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToRoleData {
    /// The unique identifier of the role.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RolesType>,
}

impl RelationshipToRoleData {
    pub fn new() -> RelationshipToRoleData {
        RelationshipToRoleData {
            id: None,
            type_: None,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::RolesType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for RelationshipToRoleData {
    fn default() -> Self {
        Self::new()
    }
}
