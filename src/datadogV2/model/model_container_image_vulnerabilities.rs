// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Vulnerability counts associated with the Container Image.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageVulnerabilities {
    /// ID of the Container Image.
    #[serde(rename = "asset_id")]
    pub asset_id: Option<String>,
    /// Number of vulnerabilities with CVSS Critical severity.
    #[serde(rename = "critical")]
    pub critical: Option<i64>,
    /// Number of vulnerabilities with CVSS High severity.
    #[serde(rename = "high")]
    pub high: Option<i64>,
    /// Number of vulnerabilities with CVSS Low severity.
    #[serde(rename = "low")]
    pub low: Option<i64>,
    /// Number of vulnerabilities with CVSS Medium severity.
    #[serde(rename = "medium")]
    pub medium: Option<i64>,
    /// Number of vulnerabilities with CVSS None severity.
    #[serde(rename = "none")]
    pub none: Option<i64>,
    /// Number of vulnerabilities with an unknown CVSS severity.
    #[serde(rename = "unknown")]
    pub unknown: Option<i64>,
}

impl ContainerImageVulnerabilities {
    pub fn new() -> ContainerImageVulnerabilities {
        ContainerImageVulnerabilities {
            asset_id: None,
            critical: None,
            high: None,
            low: None,
            medium: None,
            none: None,
            unknown: None,
        }
    }
}
impl Default for ContainerImageVulnerabilities {
    fn default() -> Self {
        Self::new()
    }
}
