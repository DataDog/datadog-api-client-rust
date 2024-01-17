// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the user object returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponseRelationships {
    /// Relationship to an organization.
    #[serde(rename = "org")]
    pub org: Option<Box<crate::datadogV2::model::RelationshipToOrganization>>,
    /// Relationship to organizations.
    #[serde(rename = "other_orgs")]
    pub other_orgs: Option<Box<crate::datadogV2::model::RelationshipToOrganizations>>,
    /// Relationship to users.
    #[serde(rename = "other_users")]
    pub other_users: Option<Box<crate::datadogV2::model::RelationshipToUsers>>,
    /// Relationship to roles.
    #[serde(rename = "roles")]
    pub roles: Option<Box<crate::datadogV2::model::RelationshipToRoles>>,
}

impl UserResponseRelationships {
    pub fn new() -> UserResponseRelationships {
        UserResponseRelationships {
            org: None,
            other_orgs: None,
            other_users: None,
            roles: None,
        }
    }
}
impl Default for UserResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}
