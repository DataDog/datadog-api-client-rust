// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentUpdateData {
    /// The incident's attributes for an update request.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: IncidentUpdateAttributes,
    /// The incident's ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The incident's relationships for an update request.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: IncidentUpdateRelationships,
    /// Incident resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: IncidentType,
}

