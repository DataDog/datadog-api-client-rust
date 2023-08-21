// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistAttributes {
    /// Whether the IP allowlist logic is enabled or not.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: bool,
    /// Array of entries in the IP allowlist.
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Vec<IPAllowlistEntry>,
}

