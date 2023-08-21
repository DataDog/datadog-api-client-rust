// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeam {
    /// Team membership attributes
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: UserTeamAttributes,
    /// The ID of a user's relationship with a team
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Relationship between membership and a user
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: UserTeamRelationships,
    /// Team membership type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: UserTeamType,
}

