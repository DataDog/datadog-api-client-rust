// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAttributes {
    /// Creation time of the organization.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Description of the organization.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Whether or not the organization is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: bool,
    /// Time of last organization modification.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// Name of the organization.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Public ID of the organization.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// Sharing type of the organization.
    #[serde(rename = "sharing", skip_serializing_if = "Option::is_none")]
    pub sharing: String,
    /// URL of the site that this organization exists at.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

