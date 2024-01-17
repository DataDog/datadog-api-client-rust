// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the IP allowlist entry.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistEntryAttributes {
    /// The CIDR block describing the IP range of the entry.
    #[serde(rename = "cidr_block")]
    pub cidr_block: Option<String>,
    /// Creation time of the entry.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Time of last entry modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// A note describing the IP allowlist entry.
    #[serde(rename = "note")]
    pub note: Option<String>,
}

impl IPAllowlistEntryAttributes {
    pub fn new() -> IPAllowlistEntryAttributes {
        IPAllowlistEntryAttributes {
            cidr_block: None,
            created_at: None,
            modified_at: None,
            note: None,
        }
    }
}
impl Default for IPAllowlistEntryAttributes {
    fn default() -> Self {
        Self::new()
    }
}
