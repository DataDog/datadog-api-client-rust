// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialAPIKey {
    /// Attributes of a partial API key.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: PartialAPIKeyAttributes,
    /// ID of the API key.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Resources related to the API key.
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: APIKeyRelationships,
    /// API Keys resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: APIKeysType,
}

