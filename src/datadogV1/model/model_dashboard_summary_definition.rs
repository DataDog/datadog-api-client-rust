// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Dashboard definition.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardSummaryDefinition {
    /// Identifier of the dashboard author.
    #[serde(rename = "author_handle")]
    pub author_handle: Option<String>,
    /// Creation date of the dashboard.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Description of the dashboard.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// Dashboard identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it.
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Layout type of the dashboard.
    #[serde(rename = "layout_type")]
    pub layout_type: Option<crate::datadogV1::model::DashboardLayoutType>,
    /// Modification date of the dashboard.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Title of the dashboard.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// URL of the dashboard.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl DashboardSummaryDefinition {
    pub fn new() -> DashboardSummaryDefinition {
        DashboardSummaryDefinition {
            author_handle: None,
            created_at: None,
            description: None,
            id: None,
            is_read_only: None,
            layout_type: None,
            modified_at: None,
            title: None,
            url: None,
        }
    }
}
