// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the IP allowlist.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistAttributes {
    /// Whether the IP allowlist logic is enabled or not.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Array of entries in the IP allowlist.
    #[serde(rename = "entries")]
    pub entries: Option<Vec<crate::datadogV2::model::IPAllowlistEntry>>,
}

impl IPAllowlistAttributes {
    pub fn new() -> IPAllowlistAttributes {
        IPAllowlistAttributes {
            enabled: None,
            entries: None,
        }
    }
}
