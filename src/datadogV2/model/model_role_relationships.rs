// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the role object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleRelationships {
    /// Relationship to multiple permissions objects.
    #[serde(rename = "permissions")]
    pub permissions: Option<crate::datadogV2::model::RelationshipToPermissions>,
    /// Relationship to users.
    #[serde(rename = "users")]
    pub users: Option<crate::datadogV2::model::RelationshipToUsers>,
}

impl RoleRelationships {
    pub fn new() -> RoleRelationships {
        RoleRelationships {
            permissions: None,
            users: None,
        }
    }

    pub fn permissions(
        &mut self,
        value: crate::datadogV2::model::RelationshipToPermissions,
    ) -> &mut Self {
        self.permissions = Some(value);
        self
    }

    pub fn users(&mut self, value: crate::datadogV2::model::RelationshipToUsers) -> &mut Self {
        self.users = Some(value);
        self
    }
}

impl Default for RoleRelationships {
    fn default() -> Self {
        Self::new()
    }
}
