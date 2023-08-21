// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListItem {
    /// Creator of the object.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Creator,
    /// Date of creation of the dashboard.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// URL to the icon of the dashboard.
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub icon: Option<String>,
    /// ID of the dashboard.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The short name of the integration.
    #[serde(rename = "integration_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub integration_id: Option<String>,
    /// Whether or not the dashboard is in the favorites.
    #[serde(rename = "is_favorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: bool,
    /// Whether or not the dashboard is read only.
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: bool,
    /// Whether the dashboard is publicly shared or not.
    #[serde(rename = "is_shared", skip_serializing_if = "Option::is_none")]
    pub is_shared: bool,
    /// Date of last edition of the dashboard.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// Popularity of the dashboard.
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: i32,
    /// List of team names representing ownership of a dashboard.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub tags: datadog.NullableList[String],
    /// Title of the dashboard.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// The type of the dashboard.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: DashboardType,
    /// URL path to the dashboard.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

