// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseRelationships {
    /// A relationship reference for attachments.
    #[serde(rename = "attachments")]
    pub attachments: RelationshipToIncidentAttachment,
    /// Relationship to user.
    #[serde(rename = "commander_user")]
    pub commander_user: NullableRelationshipToUser,
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: RelationshipToUser,
    /// A relationship reference for multiple integration metadata objects.
    #[serde(rename = "integrations")]
    pub integrations: RelationshipToIncidentIntegrationMetadatas,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: RelationshipToUser,
}

