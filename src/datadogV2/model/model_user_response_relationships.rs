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
    pub org: Option<crate::datadogV2::model::RelationshipToOrganization>,
    /// Relationship to organizations.
    #[serde(rename = "other_orgs")]
    pub other_orgs: Option<crate::datadogV2::model::RelationshipToOrganizations>,
    /// Relationship to users.
    #[serde(rename = "other_users")]
    pub other_users: Option<crate::datadogV2::model::RelationshipToUsers>,
    /// Relationship to roles.
    #[serde(rename = "roles")]
    pub roles: Option<crate::datadogV2::model::RelationshipToRoles>,
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

    pub fn with_org(
        &mut self,
        value: crate::datadogV2::model::RelationshipToOrganization,
    ) -> &mut Self {
        self.org = Some(value);
        self
    }

    pub fn with_other_orgs(
        &mut self,
        value: crate::datadogV2::model::RelationshipToOrganizations,
    ) -> &mut Self {
        self.other_orgs = Some(value);
        self
    }

    pub fn with_other_users(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUsers,
    ) -> &mut Self {
        self.other_users = Some(value);
        self
    }

    pub fn with_roles(&mut self, value: crate::datadogV2::model::RelationshipToRoles) -> &mut Self {
        self.roles = Some(value);
        self
    }
}
impl Default for UserResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}
