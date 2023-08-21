// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingUpdateData {
    /// Key/Value pair of attributes used for update request.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: AuthNMappingUpdateAttributes,
    /// ID of the AuthN Mapping.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Relationship of AuthN Mapping update object to Role.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: AuthNMappingUpdateRelationships,
    /// AuthN Mappings resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: AuthNMappingsType,
}

