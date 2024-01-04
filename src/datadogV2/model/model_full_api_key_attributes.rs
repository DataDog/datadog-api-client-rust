// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of a full API key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullAPIKeyAttributes {
    /// The category of the API key.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Creation date of the API key.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The API key.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The last four characters of the API key.
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    /// Date the API key was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Name of the API key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The remote config read enabled status.
    #[serde(rename = "remote_config_read_enabled")]
    pub remote_config_read_enabled: Option<bool>,
}

impl FullAPIKeyAttributes {
    pub fn new() -> FullAPIKeyAttributes {
        FullAPIKeyAttributes {
            category: None,
            created_at: None,
            key: None,
            last4: None,
            modified_at: None,
            name: None,
            remote_config_read_enabled: None,
        }
    }
}