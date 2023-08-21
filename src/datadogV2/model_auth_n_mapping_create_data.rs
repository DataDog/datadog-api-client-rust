// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingCreateData {
    /// Key/Value pair of attributes used for create request.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: AuthNMappingCreateAttributes,
    /// Relationship of AuthN Mapping create object to Role.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: AuthNMappingCreateRelationships,
    /// AuthN Mappings resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: AuthNMappingsType,
}

