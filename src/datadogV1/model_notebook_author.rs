// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookAuthor {
    /// Creation time of the user.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Whether the user is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: bool,
    /// Email of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Handle of the user.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// URL of the user's icon.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: String,
    /// Name of the user.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// Status of the user.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// Title of the user.
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub title: Option<String>,
    /// Whether the user is verified.
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: bool,
}

