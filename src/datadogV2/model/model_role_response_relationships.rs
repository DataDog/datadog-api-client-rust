// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the role object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleResponseRelationships {
    /// Relationship to multiple permissions objects.
    #[serde(rename = "permissions")]
    pub permissions: Option<crate::datadogV2::model::RelationshipToPermissions>,
}

impl RoleResponseRelationships {
    pub fn new() -> RoleResponseRelationships {
        RoleResponseRelationships { permissions: None }
    }

    pub fn permissions(
        &mut self,
        value: crate::datadogV2::model::RelationshipToPermissions,
    ) -> &mut Self {
        self.permissions = Some(value);
        self
    }
}

impl Default for RoleResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}
