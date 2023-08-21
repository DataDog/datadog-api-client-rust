// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamCreate {
    /// Team creation attributes
    #[serde(rename = "attributes")]
    pub attributes: TeamCreateAttributes,
    /// Relationships formed with the team on creation
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: TeamCreateRelationships,
    /// Team type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: TeamType,
}

