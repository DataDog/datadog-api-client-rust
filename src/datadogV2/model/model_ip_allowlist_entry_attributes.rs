// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistEntryAttributes {
    /// The CIDR block describing the IP range of the entry.
    #[serde(rename = "cidr_block", skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// Creation time of the entry.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Time of last entry modification.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// A note describing the IP allowlist entry.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl IPAllowlistEntryAttributes {
    /// Attributes of the IP allowlist entry.
    pub fn new() -> IPAllowlistEntryAttributes {
        IPAllowlistEntryAttributes {
            cidr_block: None,
            created_at: None,
            modified_at: None,
            note: None,
        }
    }
}
