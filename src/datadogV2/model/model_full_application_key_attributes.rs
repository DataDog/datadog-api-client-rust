// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of a full application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullApplicationKeyAttributes {
    /// Creation date of the application key.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The application key.
    #[serde(rename = "key")]
    pub key: Option<String>,
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

impl FullApplicationKeyAttributes {
    pub fn new() -> FullApplicationKeyAttributes {
        FullApplicationKeyAttributes {
            created_at: None,
            key: None,
            last4: None,
            name: None,
            scopes: None,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn last4(mut self, value: String) -> Self {
        self.last4 = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn scopes(mut self, value: Option<Vec<String>>) -> Self {
        self.scopes = Some(value);
        self
    }
}

impl Default for FullApplicationKeyAttributes {
    fn default() -> Self {
        Self::new()
    }
}
