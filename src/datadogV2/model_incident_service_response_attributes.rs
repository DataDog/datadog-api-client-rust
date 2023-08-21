// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceResponseAttributes {
    /// Timestamp of when the incident service was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// Timestamp of when the incident service was modified.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// Name of the incident service.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}

