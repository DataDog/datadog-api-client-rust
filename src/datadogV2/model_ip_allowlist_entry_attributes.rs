// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistEntryAttributes {
    /// The CIDR block describing the IP range of the entry.
    #[serde(rename = "cidr_block", skip_serializing_if = "Option::is_none")]
    pub cidr_block: String,
    /// Creation time of the entry.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Time of last entry modification.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// A note describing the IP allowlist entry.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: String,
}

