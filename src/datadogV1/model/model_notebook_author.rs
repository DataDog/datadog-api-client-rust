// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of user object returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookAuthor {
    /// Creation time of the user.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Whether the user is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// Email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the user.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// URL of the user's icon.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// Name of the user.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// Status of the user.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Title of the user.
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option")]
    pub title: Option<Option<String>>,
    /// Whether the user is verified.
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
}

impl NotebookAuthor {
    pub fn new() -> NotebookAuthor {
        NotebookAuthor {
            created_at: None,
            disabled: None,
            email: None,
            handle: None,
            icon: None,
            name: None,
            status: None,
            title: None,
            verified: None,
        }
    }
}
impl Default for NotebookAuthor {
    fn default() -> Self {
        Self::new()
    }
}
