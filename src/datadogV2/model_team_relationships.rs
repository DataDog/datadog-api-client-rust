// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamRelationships {
    /// Relationship between a team and a team link
    #[serde(rename = "team_links", skip_serializing_if = "Option::is_none")]
    pub team_links: RelationshipToTeamLinks,
    /// Relationship between a user team permission and a team
    #[serde(rename = "user_team_permissions", skip_serializing_if = "Option::is_none")]
    pub user_team_permissions: RelationshipToUserTeamPermission,
}

