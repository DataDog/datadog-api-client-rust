// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListItem {
    /// Creator of the object.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<crate::datadogV2::model::Creator>>,
    /// Date of creation of the dashboard.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// URL to the icon of the dashboard.
    #[serde(
        rename = "icon",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon: Option<Option<String>>,
    /// ID of the dashboard.
    #[serde(rename = "id")]
    pub id: String,
    /// The short name of the integration.
    #[serde(
        rename = "integration_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_id: Option<Option<String>>,
    /// Whether or not the dashboard is in the favorites.
    #[serde(rename = "is_favorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    /// Whether or not the dashboard is read only.
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    /// Whether the dashboard is publicly shared or not.
    #[serde(rename = "is_shared", skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    /// Date of last edition of the dashboard.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    /// Popularity of the dashboard.
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    /// List of team names representing ownership of a dashboard.
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    /// Title of the dashboard.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of the dashboard.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DashboardType,
    /// URL path to the dashboard.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DashboardListItem {
    /// A dashboard within a list.
    pub fn new(id: String, type_: crate::datadogV2::model::DashboardType) -> DashboardListItem {
        DashboardListItem {
            author: None,
            created: None,
            icon: None,
            id: id,
            integration_id: None,
            is_favorite: None,
            is_read_only: None,
            is_shared: None,
            modified: None,
            popularity: None,
            tags: None,
            title: None,
            type_: type_,
            url: None,
        }
    }
}
