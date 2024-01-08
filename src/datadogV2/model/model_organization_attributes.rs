// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAttributes {
    /// Creation time of the organization.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Description of the organization.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether or not the organization is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// Time of last organization modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Name of the organization.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Public ID of the organization.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Sharing type of the organization.
    #[serde(rename = "sharing")]
    pub sharing: Option<String>,
    /// URL of the site that this organization exists at.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl OrganizationAttributes {
    pub fn new() -> OrganizationAttributes {
        OrganizationAttributes {
            created_at: None,
            description: None,
            disabled: None,
            modified_at: None,
            name: None,
            public_id: None,
            sharing: None,
            url: None,
        }
    }
}
impl Default for OrganizationAttributes {
    fn default() -> Self {
        Self::new()
    }
}
