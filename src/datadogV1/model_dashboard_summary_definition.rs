// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardSummaryDefinition {
    /// Identifier of the dashboard author.
    #[serde(rename = "author_handle", skip_serializing_if = "Option::is_none")]
    pub author_handle: String,
    /// Creation date of the dashboard.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Description of the dashboard.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Dashboard identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it.
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: bool,
    /// Layout type of the dashboard.
    #[serde(rename = "layout_type", skip_serializing_if = "Option::is_none")]
    pub layout_type: DashboardLayoutType,
    /// Modification date of the dashboard.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// Title of the dashboard.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// URL of the dashboard.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

