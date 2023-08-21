// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponseRelationships {
    /// Relationship to an organization.
    #[serde(rename = "org")]
    pub org: RelationshipToOrganization,
    /// Relationship to organizations.
    #[serde(rename = "other_orgs")]
    pub other_orgs: RelationshipToOrganizations,
    /// Relationship to users.
    #[serde(rename = "other_users")]
    pub other_users: RelationshipToUsers,
    /// Relationship to roles.
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: RelationshipToRoles,
}

