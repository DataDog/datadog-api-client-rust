// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocation {
    /// Description of the private location.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Unique identifier of the private location.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Object containing metadata about the private location.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: SyntheticsPrivateLocationMetadata,
    /// Name of the private location.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Secrets for the private location. Only present in the response when creating the private location.
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: SyntheticsPrivateLocationSecrets,
    /// Array of tags attached to the private location.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

