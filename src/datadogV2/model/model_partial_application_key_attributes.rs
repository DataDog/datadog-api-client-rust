// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of a partial application key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialApplicationKeyAttributes {
    /// Creation date of the application key.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The last four characters of the application key.
    #[serde(rename = "last4")]
    pub last4: Option<String>,
    /// Name of the application key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option")]
    pub scopes: Option<Option<Vec<String>>>,
}

impl PartialApplicationKeyAttributes {
    pub fn new() -> PartialApplicationKeyAttributes {
        PartialApplicationKeyAttributes {
            created_at: None,
            last4: None,
            name: None,
            scopes: None,
        }
    }
}
impl Default for PartialApplicationKeyAttributes {
    fn default() -> Self {
        Self::new()
    }
}
