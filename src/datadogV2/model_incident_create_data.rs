// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCreateData {
    /// The incident's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: IncidentCreateAttributes,
    /// The relationships the incident will have with other resources once created.
    #[serde(rename = "relationships")]
    pub relationships: IncidentCreateRelationships,
    /// Incident resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: IncidentType,
}

