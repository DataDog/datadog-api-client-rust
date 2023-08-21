// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialAPIKeyAttributes {
    /// Creation date of the API key.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// The last four characters of the API key.
    #[serde(rename = "last4", skip_serializing_if = "Option::is_none")]
    pub last4: String,
    /// Date the API key was last modified.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// Name of the API key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}

