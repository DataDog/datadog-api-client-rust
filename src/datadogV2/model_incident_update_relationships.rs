// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentUpdateRelationships {
    /// Relationship to user.
    #[serde(rename = "commander_user")]
    pub commander_user: NullableRelationshipToUser,
    /// A relationship reference for multiple integration metadata objects.
    #[serde(rename = "integrations")]
    pub integrations: RelationshipToIncidentIntegrationMetadatas,
    /// A relationship reference for postmortems.
    #[serde(rename = "postmortem")]
    pub postmortem: RelationshipToIncidentPostmortem,
}

